use std::fs;

/*
#[derive(Debug)]
enum Node<'a> {
    File(&'a str),
    Directory(&'a str, Vec<Node<'a>>),
}

impl<'a> Node <'a> {
    fn new_file(filename: &'a str) -> Self {
        Node::File(filename)
    }
    
    fn new_empty_directory(directoryname :&'a str, children: Vec<Node<'a>>) -> Self {
        Node::Directory(directoryname, children)
    }
    
    fn get_children(&self) -> &Vec<Node<'a>> {
        match self {
            Node::Directory(_,_) => {
                self.children
            }
            Node::File(_,_) => {
                None
            }
        }
    }
}
*/
enum FileSystem <'a> {
    Directory(Directory<'a>),
    File(File<'a>)
}

struct Directory <'a>{
    name : &'a str,
    children : Vec<FileSystem<'a>>
}

struct File <'a> {
    name : &'a str
}

trait Name {
    fn name(&self) -> &str;
}

trait Parent {
    fn get_children(&self) -> &Vec<FileSystem>;
}

impl<'a> Name for Directory<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Name for File<'a> {
    fn name(&self) -> &str {
        self.name
    }
}

impl<'a> Parent for Directory<'a> {
    fn get_children(&self) -> &Vec<FileSystem<'a>> {
        &self.children
    }
}


fn main() {
    /*
    let data = fs::read("../README.md").expect("Unable to open file");
    println!("{}", data.len());
    let data = String::from_utf8(data).expect("Couldn't parse UTF 8");
    println!("{data}");
    */
    
}
