use alloc::string::String;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone)]
pub struct VfsNode {
    pub name: String,
    pub node_type: NodeType,
    pub content: Vec<u8>,
    pub children: Vec<VfsNode>,
}

impl VfsNode {
    pub fn new_file(name: &str, content: &[u8]) -> Self {
        VfsNode {
            name: String::from(name),
            node_type: NodeType::File,
            content: content.to_vec(),
            children: Vec::new(),
        }
    }

    pub fn new_directory(name: &str) -> Self {
        VfsNode {
            name: String::from(name),
            node_type: NodeType::Directory,
            content: Vec::new(),
            children: Vec::new(),
        }
    }
}

pub struct VirtualFileSystem {
    root: VfsNode,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        let mut root = VfsNode::new_directory("/");
        root.children.push(VfsNode::new_file(
            "welcome.txt",
            b"Welcome to Zenith OS VFS Filesystem!",
        ));
        root.children.push(VfsNode::new_directory("bin"));
        root.children.push(VfsNode::new_directory("sys"));

        VirtualFileSystem { root }
    }

    pub fn read_file(&self, filename: &str) -> Option<Vec<u8>> {
        let clean_name = filename.trim_start_matches('/');
        for child in &self.root.children {
            if child.name == clean_name && child.node_type == NodeType::File {
                return Some(child.content.clone());
            }
        }
        None
    }

    pub fn write_file(&mut self, filename: &str, content: &[u8]) -> bool {
        let clean_name = filename.trim_start_matches('/');
        for child in &mut self.root.children {
            if child.name == clean_name && child.node_type == NodeType::File {
                child.content = content.to_vec();
                return true;
            }
        }
        // Create new file if it doesn't exist
        self.root
            .children
            .push(VfsNode::new_file(clean_name, content));
        true
    }

    pub fn list_root(&self) -> Vec<String> {
        let mut list = Vec::new();
        for child in &self.root.children {
            let prefix = if child.node_type == NodeType::Directory {
                "[DIR] "
            } else {
                "[FILE] "
            };
            list.push(String::from(prefix) + &child.name);
        }
        list
    }
}

lazy_static! {
    pub static ref VFS: Mutex<VirtualFileSystem> = Mutex::new(VirtualFileSystem::new());
}
