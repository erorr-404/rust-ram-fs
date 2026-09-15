use std::io;


enum NodeType {
    File {
        content: String,
        size_bytes: usize
    },

    Directory {
        name: String,
        nodes: Vec<u32>
    }
}

struct Node {
    id: u32,
    node_type: NodeType,
    created_at_step: u32
}

struct VirtualDisk {
    nodes: Vec<Node>,
    capacity: usize,
    current_step: u32
}

impl VirtualDisk {
    fn new(capacity: usize) -> VirtualDisk {
        VirtualDisk {
            nodes: Vec::new(),
            capacity,
            current_step: 0
        }
    }

    fn add_node(&mut self, node: Node) -> bool {
        if self.nodes.len() >= self.capacity { return false; }
        self.nodes.push(node);
        true
    }

    fn find_by_id(&self, id: u32) -> Option<&Node> {
        for node in self.nodes.iter() {
            if node.id == id {
                return Some(node);
            }
        }
        return None;
    }
}

fn touch(args: &[&str], disk: &mut VirtualDisk) {
    todo!("");
}

fn mkdir(args: &[&str], disk: &mut VirtualDisk) {
    todo!("");
}

fn cat(args: &[&str], disk: &VirtualDisk) {
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

        // get imput from user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can not read line");
        let comand_parts: Vec<&str> = input.trim().split(' ').collect();

        // error detection
        if comand_parts.is_empty() || comand_parts[0].is_empty() {
            continue;
        }
        
        // convert imput to command and arguments
        let command = &comand_parts[0];
        let arguments = &comand_parts[1..];

        // execute command with given arguments
        match *command {
            "touch" => touch(&arguments, &mut fs),
            "mkdir" => mkdir(&arguments, &mut fs),
            "cat" => cat(&arguments, &fs),
            "rm" => rm(&arguments, &mut fs),
            _ => println!("Invalid command")
        }
    }
}
