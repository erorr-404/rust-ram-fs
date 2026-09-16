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

    fn find_by_id(&self, id: u32) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn find_by_id_mut(&mut self, id: u32) -> Option<&mut Node> {
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

    fn get_current_folder(&mut self) -> &mut NodeType {
        let current_node = match self.disk.find_by_id_mut(self.disk._next_node_id) {
            Some(node) => match node.node_type {
                NodeType::Directory { .. } => node,
                NodeType::File { .. } => {panic!("WTF: current folder is File")}
            },
            None => {panic!("WTF: current folder is None")}
        };

        &mut current_node.node_type
    }

    fn create_file(&mut self, name: String, content: String, folder: &mut NodeType) {
        let node_id = self.disk.add_node(Node::new(NodeType::new_file(content), &self.disk));
        NodeType::add_file_to_folder(name, node_id, folder);
    }

    fn create_folder(&mut self, name: String, folder: &mut NodeType) {
        NodeType::new_empty_folder(Some(self.disk.current_dir_id));
    }
}

fn touch(args: &[&str], disk: &mut VirtualDisk) {
    if args.len() != 2 {
        println!("Invalid syntax. Expected: touch <name> <content>");
        return;
    }

    let node = Node {
        node_type: NodeType::File { 
            content: args[1].to_string(), 
            size_bytes: args[1].len() 
        },
        created_at_step: disk._current_step
    };

    let file_name = args[0].to_string();
    let this_node_id = disk._next_node_id;
    let res = disk.add_node(node);
    

        let parent_folder: &mut Node = match disk.find_by_id_mut(disk.current_dir_id) {
            Some(f) => f,
            None => {panic!("WTF, pwd is None, LOL.");}
        };
        
        match parent_folder.node_type {
            NodeType::Directory { ref mut nodes, .. } => {
                nodes.insert(file_name, this_node_id);
            },
            NodeType::File {..} => {
                panic!("WFT?? Parent folder is FILE.");
            }
        }
        
        println!("File successfully created.");
        return;
    
    
}

fn cat(args: &[&str], disk: &VirtualDisk) {
    if args.len() != 1 {
        println!("Invalid syntax. Expected: cat <node_id>.");
        return;
    }

    let node_id: u32 = match args[0].parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid id.");
            return;
        }
    };

    let node: &Node = match disk.find_by_id(node_id) {
        Some(n) => n,
        None => {
            println!("Node does not exist.");
            return;
        }
    };

    match &node.node_type {
        NodeType::File { content, size_bytes } => {
            println!("File size: {} bytes.", size_bytes);
            println!("Content:\n{}", content);
        },
        NodeType::Directory {..} => {
            println!("This is a directory.");
        }
    }
}

fn mkdir(args: &[&str], disk: &mut VirtualDisk) {

    if args.len() != 1 {
        println!("Invalid syntax. Example: mkdir <name> .");
        return;
    }

    let dir = Node {
        node_type: NodeType::Directory { 
            nodes: HashMap::new(),
            parent_id: Some(disk.current_dir_id)
        },
        created_at_step: disk._current_step  
    };

    let res = disk.add_node(dir);

    println!("Directory successfully created.");
}

fn ls(args: &[&str], disk: &mut VirtualDisk) {
    todo!("");
}

fn cd(args: &[&str], disk: &mut VirtualDisk) {
    todo!("");
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
    
    let mut fs = VirtualDisk::new(size);

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
            "ls" => ls(&arguments, &mut fs),
            "cd" => cd(&arguments, &mut fs),
            "rm" => rm(&arguments, &mut fs),
            _ => println!("Invalid command")
        }
    }
}
