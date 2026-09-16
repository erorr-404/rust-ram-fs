use std::fs::File;
use std::io;
use std::collections::HashMap;

enum NodeType {
    File {
        content: String,
        size_bytes: usize
    },

    Directory {
        nodes: HashMap<String, u32>, // key: file name, value: node_id
        parent_id: Option<u32>
    }
}

impl NodeType {
    fn new_file(content: String) -> NodeType {
        NodeType::File {
            size_bytes: content.len(),
            content, 
        }
    }

    fn new_directory(nodes: HashMap<String, u32>, parent_id: Option<u32>) -> NodeType {
        NodeType::Directory { 
            nodes, 
            parent_id 
        }
    }

    fn new_empty_folder(parent_id: Option<u32>) -> NodeType {
        NodeType::Directory { 
            nodes: HashMap::new(), 
            parent_id 
        }
    }

    fn add_file_to_folder(name: String, file_node_id: u32, folder: &mut NodeType) {
        match folder {
            &mut NodeType::Directory { ref mut nodes, .. } => {
                nodes.insert(name, file_node_id);
            },
            &mut NodeType::File { .. } => {panic!("Folder can not be a file")}
        }
    }
}

struct Node {
    node_type: NodeType,
    created_at_step: u32
}

impl Node {
    fn new(node_type: NodeType, disk: &VirtualDisk) -> Node {
        Node { 
            node_type, 
            created_at_step: disk._current_step 
        }
    }
}

struct VirtualDisk {
    nodes: HashMap<u32, Node>,
    _current_step: u32,
    current_dir_id: u32,
    _next_node_id: u32
}

impl VirtualDisk {
    fn new(node_count: usize) -> VirtualDisk {
        let mut vd = VirtualDisk {
            nodes: HashMap::new(),
            _current_step: 0,
            current_dir_id: 0,
            _next_node_id: 0
        };

        vd.add_node(
            Node { 
                node_type: NodeType::Directory { 
                    nodes: HashMap::new(), 
                    parent_id: None }, 
                created_at_step: 0 
            }
        );
        
        vd
    }

    fn add_node(&mut self, node: Node) -> u32 {
        self.nodes.insert(self._next_node_id, node);
        let cur_node = self._next_node_id;
        self.increase_next_node_id();
        return cur_node;
    }

    fn increase_next_node_id(&mut self) {
        self._next_node_id += 1; // TODO: check if it exists
    }

    fn find_by_id(&self, id: &u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn find_by_id_mut(&mut self, id: &u32) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }
}

struct FileSystem {
    disk: VirtualDisk
}

impl FileSystem {
    fn new() -> FileSystem{
        FileSystem {
            disk: VirtualDisk::new(4000)
        }
    }

    fn get_current_folder(&self) -> &NodeType {
        let current_node = match self.disk.find_by_id(&self.disk.current_dir_id) {
            Some(node) => node,
            None => panic!("WTF: current folder is None"),
        };

        match &current_node.node_type {
            NodeType::Directory { .. } => &current_node.node_type,
            NodeType::File { .. } => panic!("WTF: current folder is File"),
        }
    }

    fn get_current_folder_mut(&mut self) -> &mut NodeType {
        let current_dir_id = self.disk.current_dir_id;
        let current_node = match self.disk.find_by_id_mut(&current_dir_id) {
            Some(node) => node,
            None => panic!("WTF: current folder is None"),
        };

        match &current_node.node_type {
            NodeType::Directory { .. } => &mut current_node.node_type,
            NodeType::File { .. } => panic!("WTF: current folder is File"),
        }
    }

    fn create_file(&mut self, name: String, content: String) {
        let node_id = self.disk.add_node(Node::new(NodeType::new_file(content), &self.disk));
        NodeType::add_file_to_folder(name, node_id, self.get_current_folder_mut());
    }

    fn create_folder(&mut self, name: String) {
        let new_folder = NodeType::new_empty_folder(Some(self.disk.current_dir_id));
        let node_id = self.disk.add_node(Node::new(new_folder, &self.disk));
        NodeType::add_file_to_folder(name, node_id, self.get_current_folder_mut());
    }

    fn read_file(&self, name: String) -> &String {
        let node_id = {
            let current_dir = self.get_current_folder();
            match current_dir {
                NodeType::Directory { nodes, .. } => {
                    *nodes.get(&name).expect("Can not find file with this name")
                },
                NodeType::File { .. } => {panic!("Current folder can not be a file.");}
            }
        };
        let content = match self.disk.find_by_id(&node_id) {
            Some(n) => {
                match &n.node_type {
                    NodeType::File { content, .. } => {content},
                    NodeType::Directory { .. } => {panic!("{name} is a directory")}
                }
            },
            None => panic!("Can not find file with this name")
        };

        content
    }
}

fn touch(args: &[&str], fs: &mut FileSystem) {
    if args.len() != 2 {
        println!("Invalid syntax. Expected: touch <name> <content>");
        return;
    }
    let file_name = args[0].to_string();
    let content = args[1].to_string();
    fs.create_file(file_name, content);
    println!("File created.");
}

fn cat(args: &[&str], fs: &FileSystem) {
    if args.len() != 1 {
        println!("Invalid syntax. Expected: cat <file_name>.");
        return;
    }
    let file_name = args[0].to_string();
    let content = fs.read_file(file_name);
    println!("{content}");
}

fn mkdir(args: &[&str], fs: &mut FileSystem) {

    if args.len() != 1 {
        println!("Invalid syntax. Example: mkdir <name> .");
        return;
    }

    let folder_name = args[0].to_string();
    fs.create_folder(folder_name);
    println!("Directory successfully created.");
}

fn ls(args: &[&str], fs: &FileSystem) {
    if !args.is_empty() {
        println!("Invalid syntax. Expected: ls.");
        return;
    }

    let mut entries: Vec<&String> = match fs.get_current_folder() {
        NodeType::Directory { nodes, .. } => nodes.keys().collect(),
        NodeType::File { .. } => return,
    };

    entries.sort();
    for entry in entries {
        println!("{entry}");
    }
}

fn cd(args: &[&str], fs: &mut FileSystem) {
    if args.len() != 1 {
        println!("Invalid syntax. Example: cd <folder_name>/<..> .");
        return;
    }

    let path = args[0];
    let current_dir_id = fs.disk.current_dir_id;
    
    if path == ".." {
        let parent_id = match fs.disk.find_by_id(&current_dir_id) {
            Some(Node { node_type: NodeType::Directory { parent_id, .. }, .. }) => *parent_id,
            _ => None,
        };

        if let Some(parent_id) = parent_id {
            fs.disk.current_dir_id = parent_id;
        } else {
            println!("Already at the root directory.");
        }
    } else {
        let child_id = match fs.get_current_folder() {
            NodeType::Directory { nodes, .. } => match nodes.get(path) {
                Some(id) => *id,
                None => {
                    println!("Directory not found: {path}");
                    return;
                }
            },
            NodeType::File { .. } => return,
        };

        match fs.disk.find_by_id(&child_id) {
            Some(Node { node_type: NodeType::Directory { .. }, .. }) => {
                fs.disk.current_dir_id = child_id;
            }
            Some(Node { node_type: NodeType::File { .. }, .. }) => {
                println!("{path} is not a directory.");
            }
            None => println!("Directory not found: {path}"),
        }
    }
}

fn rm(args: &[&str], disk: &mut VirtualDisk) {
    todo!("");
}

fn main() {
    println!("Enter VirtualDisk size: ");
    let mut size = String::new();
    
    io::stdin()
        .read_line(&mut size)
        .expect("Can not read stdin");
    
    let size: usize = match size.trim().parse() {
        Ok(s) => s,
        Err(_) => panic!("Invalid size")
    };
    
    let mut fs = FileSystem::new();

    loop {

        print!("> ");

        // get input from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can not read line");
        let command_parts: Vec<&str> = input.trim().split(' ').collect();

        // error detection
        if command_parts.is_empty() || command_parts[0].is_empty() {
            continue;
        }
        
        // convert input to command and arguments
        let command = &command_parts[0];
        let arguments = &command_parts[1..];

        // execute command with given arguments
        match *command {
            "touch" => touch(&arguments, &mut fs),
            "mkdir" => mkdir(&arguments, &mut fs),
            "cat" => cat(&arguments, &fs),
            "ls" => ls(&arguments, &fs),
            "cd" => cd(&arguments, &mut fs),
            // "rm" => rm(&arguments, &mut fs),
            _ => println!("Invalid command")
        }
    }
}
