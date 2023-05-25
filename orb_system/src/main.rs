use std::fs;

#[derive(Debug)]
struct File <'a>{
    filename: &'a str,
}

#[derive(Debug)]
enum Node {
    File(File),
    Directory(String, Vec<Node>),
}

impl Node {
    fn new_file(filename: &str) -> Node {
        Node::File(File { filename })
    }

    fn new_directory(name: String, children: Vec<Node>) -> Node {
        Node::Directory(name, children)
    }
}


fn main() {
    let data = fs::read("../README.md").expect("Unable to open file");
    println!("{}", data.len());
    let data = String::from_utf8(data).expect("Couldn't parse UTF 8");

    println!("{data}");
}
