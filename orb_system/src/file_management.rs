
use std::fs::{self, FileType};



pub struct File {
    pub path : String,  
    pub data : Vec<u8>
}

pub struct Directory{
    //Idem que pour le File
    pub path : String,
    pub children_directory : Vec<Directory>,
    pub children_file : Vec<File>
}

impl File{
    pub fn new_empty(path: &str) -> Self{
        File { path : path.to_owned(), data: Vec::new()}
    }

    pub fn new(path : &str, data: Vec<u8>) -> Self{
        File { path : path.to_owned(), data: data }
    }

    pub fn get_name(&self) -> &str {
        self.path.split('/').last().expect("Couldn't get the name of the file")
    }

    pub fn print_file_content_as_str(&self){
        println!("{}", String::from_utf8_lossy(&self.data));
    }

    pub fn read_data_from_file(&mut self){
        self.data = std::fs::read(&self.path).expect("Unable to open file");
    }

    pub fn empty_data_of_file(&mut self){
        self.data.clear();
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        //On créé le fichier
        let mut binary_file : Vec<u8> = Vec::new();

        //On ajoute la taille du path au fichier
        let len_path : u64 = self.path.len() as u64;
        let len_path : [u8;8] = len_path.to_le_bytes();
        binary_file.extend(len_path); //len_path est moved du coup deallocated

        //On ajoute le path au fichier
        binary_file.extend(self.path.as_bytes());

        //On lit la data du fichier et on ajoute la taille de la data
        self.data.clear();
        self.read_data_from_file();
        let len_data : u64 = self.data.len() as u64;
        let len_data : [u8;8] = len_data.to_le_bytes();
        binary_file.extend(len_data); //len_data est moved du coup deallocated

        //On ajoute ensuite la data au fichier
        binary_file.extend(&self.data);
        self.data.clear(); //Pour nettoyer un peu la mémoire et pouvoir gérer plusieurs fichiers a peu près gros (ne dépassant pas la RAM)

        //println!("{:?}", binary_file);
        binary_file
    }

    pub fn deserialize(binary : Vec<u8>) -> File {
        let mut offset : usize = 0;
        //TODO : On verifiera la taille du fichier plus tard ...

        //On récupère la taille du path
        let mut len_path_byte : [u8;8] = [0;8];
        len_path_byte.copy_from_slice(&binary[0..8]);
        let len_path : usize = u64::from_le_bytes(len_path_byte) as usize;
        offset += 8;

        //On récupère le path
        let mut path_bytes : Vec<u8> = Vec::new();
        for i in offset..offset + len_path {
            path_bytes.push(binary[i]);
        }
        let path : String = String::from_utf8(path_bytes).expect("Invalid UTF-8 value");
        offset += len_path;

        //On récupère la longueur de la data
        let mut len_data_byte : [u8;8] = [0;8];
        len_data_byte.copy_from_slice(&binary[offset..offset+8]);
        let len_data : usize = u64::from_le_bytes(len_data_byte) as usize;
        offset += 8;

        //On récupère la data
        let mut data : Vec<u8> = Vec::new();
        for i in offset..offset + len_data {
            data.push(binary[i]);
        }

        File::new(&path, data)
    }
}

impl Directory{
    pub fn new_empty(path : &str) -> Self {
        Directory { path : path.to_owned(), children_directory: Vec::new(), children_file: Vec::new() }
    }

    pub fn get_name(&self) -> &str {
        self.path.split('/').last().expect("Couldn't get the name of the directory")
    }

    pub fn add_directory(&mut self, directory : Directory){
        self.children_directory.push(directory)
    }

    pub fn add_file(&mut self, file : File){
        self.children_file.push(file)
    }

    pub fn explore_hierarchy(&mut self){
        //On vide d'abord les fichiers et dossiers dans le dossier
        for _ in 0..self.children_directory.len() {
            self.children_directory.pop();
        }
        for _ in 0..self.children_file.len() {
            self.children_file.pop();
        }

        match fs::read_dir(&self.path){
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(valid_entry) => {
                            let file_type : FileType = valid_entry
                                                        .file_type()
                                                        .expect("Error getting filetype");
                            if file_type.is_dir() {
                                let mut dir : Directory = Directory::new_empty(valid_entry
                                                                            .path()
                                                                            .to_str()
                                                                            .expect("Error getting path"));
                                dir.explore_hierarchy();
                                self.add_directory(dir)
                            }
                            else if file_type.is_file() {
                                self.add_file(File::new_empty(valid_entry
                                                                .path()
                                                                .to_str()
                                                                .expect("Error getting path")));
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

    pub fn print_dir(&self, indent_lvl : usize){
        let path_split: Vec<&str> = self.path.split('/').collect();
        let name = path_split[path_split.len()-1];

        println!("{}└ {}", "  ".repeat(indent_lvl), &name);
        for child in &self.children_directory{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.get_name());
        }
        for child in &self.children_file{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.get_name());
        }
    }

    pub fn print_arch(&self){
        self.print_arch_rec(0);
    }

    fn print_arch_rec(&self, indent_lvl : usize){
        println!("{}└ {}", "  ".repeat(indent_lvl), self.get_name());
        for child in &self.children_directory{
            child.print_arch_rec(indent_lvl+1);
        }
        for child in &self.children_file{
            println!("{}└ {}", "  ".repeat(indent_lvl+1), child.get_name());
        }
    }
}