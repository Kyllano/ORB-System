
use std::fs::{self, FileType};



pub struct File <'a> {
    pub path : &'a str,
    pub data : Vec<u8>
}

pub struct Directory <'a>{
    pub path : &'a str,
    pub children_directory : Vec<Directory<'a>>,
    pub children_file : Vec<File<'a>>
}

impl<'a> File<'a>{
    pub fn new_empty(path: &'a str) -> Self{
        File { path : path, data: Vec::new()}
    }

    pub fn new(path : &'a str, data: Vec<u8>) -> Self{
        File { path : path, data: data }
    }

    pub fn print_file_content_as_str(&self){
        println!("{}", String::from_utf8_lossy(&self.data));
    }

    pub fn read_data_from_file(&mut self, path_to_file : &str){
        self.data = std::fs::read(path_to_file).expect("Unable to open file");
    }
}

impl<'a> Directory<'a>{
    pub fn new_empty(path : &'a str) -> Self {
        Directory { path : path.clone(), children_directory: Vec::new(), children_file: Vec::new() }
    }

    pub fn add_directory(&mut self, directory : Directory<'a>){
        self.children_directory.push(directory)
    }

    pub fn add_file(&mut self, file : File<'a>){
        self.children_file.push(file)
    }

    pub fn explore_hierarchy(&mut self){
        match fs::read_dir(self.path){
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(valid_entry) => {
                            let file_type : FileType = valid_entry
                                                        .file_type()
                                                        .expect("Error getting filetype");
                            if file_type.is_dir() {
                                let dir_path = valid_entry.path();
                                let path_str : &str = dir_path.to_str().expect("prout");
                                self.add_directory(Directory::new_empty(path_str));
                            }
                            else if file_type.is_file() {
                                println!("prout");
                            }
                            else {
                                println!("File type for {} is not supported", valid_entry
                                                                            .file_name()
                                                                            .to_str()
                                                                            .expect("cannot parse filename"));
                            }
                        }
                        Err(error) => {
                            println!("Error {}", error);
                        }
                    }
                }
            }
            Err(error) =>{
                println!("Error {}", error);
            }
        }
    }

    pub fn _print_dir(&self, indent_lvl : usize){
        let path_split: Vec<&str> = self.path.split('/').collect();
        let name = path_split[path_split.len()-1];

        println!("{}└ {}", "  ".repeat(indent_lvl), &name);
        for child in &self.children_directory{
            let path_split_child_dir: Vec<&str> = child.path.split('/').collect();
            let name_child_dir = path_split_child_dir[path_split.len()-1];
            println!("{}└ {}", "  ".repeat(indent_lvl+1), &name_child_dir);
        }
        for child in &self.children_file{
            let path_split_child_file: Vec<&str> = child.path.split('/').collect();
            let name_child_file = path_split_child_file[path_split.len()-1];
            println!("{}└ {}", "  ".repeat(indent_lvl+1), &name_child_file);
        }
    }

    pub fn print_arch(&self){
        self.print_arch_rec(0);
    }

    fn print_arch_rec(&self, indent_lvl : usize){
        let path_split: Vec<&str> = self.path.split('/').collect();
        let name = path_split[path_split.len()-1];

        println!("{}└ {}", "  ".repeat(indent_lvl), &name);
        for child in &self.children_directory{
            child.print_arch_rec(indent_lvl+1);
        }
        for child in &self.children_file{
            let path_split_child_file: Vec<&str> = child.path.split('/').collect();
            let name_child_file = path_split[path_split.len()-1];
            println!("{}└ {}", "  ".repeat(indent_lvl+1), name_child_file);
        }
    }
}