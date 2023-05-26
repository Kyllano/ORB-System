use std::fs;

struct File <'a> {
    name : &'a str,
    data : Vec<u8>
}

struct Directory <'a>{
    name : &'a str,
    children_directory : Vec<Directory<'a>>,
    children_file : Vec<File<'a>>
}

impl<'a> File<'a>{
    fn new_empty(file_name: &'a str) -> Self{
        File { name: file_name , data: Vec::new()}
    }

    fn new(file_name: &'a str, data: Vec<u8>) -> Self{
        File { name: file_name, data: data }
    }
}

impl<'a> Directory<'a>{
    fn new_empty(directory_name : &'a str) -> Self {
        Directory { name: directory_name, children_directory: Vec::new(), children_file: Vec::new() }
    }

    fn add_directory(&mut self, directory : Directory<'a>){
        self.children_directory.push(directory)
    }

    fn add_file(&mut self, file : File<'a>){
        self.children_file.push(file)
    }

    fn print_arch(&self, indent_lvl : usize){
        println!("{}└ {}", "  ".repeat(indent_lvl), self.name);
        for child in &self.children_directory{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.name);
        }
        for child in &self.children_file{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.name);
        }
    }

    fn print_arch_rec(&self, indent_lvl : usize){
        println!("{}└ {}", "  ".repeat(indent_lvl), self.name);
        for child in &self.children_directory{
            child.print_arch_rec(indent_lvl+1);
        }
        for child in &self.children_file{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.name);
        }
    }
}

fn main() {
    
    let data = fs::read("../README.md").expect("Unable to open file");
    

    let mut root = Directory::new_empty("root");
    root.add_directory(Directory::new_empty("subfolder"));
    root.add_file(File::new("lol", data));
    root.add_file(File::new_empty("prout"));
    root.children_directory[0].add_file(File::new_empty("homework"));

    root.print_arch_rec(0);
    
}
