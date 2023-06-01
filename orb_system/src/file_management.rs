
use std::{fs::{self, FileType}, error::Error};


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
        binary_file
    }

    pub fn deserialize(binary : &Vec<u8>) -> Result<File, String> {
        Self::deserialize_f(binary, 0)
    }

    fn deserialize_f(binary : &Vec<u8>, offset_f : usize) -> Result<File, String> {
        let mut offset : usize = offset_f;

        //On récupère la taille du path
        let len_path : usize = deserialize_u64(&binary, &offset)? as usize;
        offset += 8;

        //On récupère le path
        if binary.len() < offset + len_path{
            return Err(String::from("Path sized written in binary is incorrect"));
        }
        let mut path_bytes : Vec<u8> = Vec::new();
        for i in offset..offset + len_path {
            path_bytes.push(binary[i]);
        }
        let path : String = String::from_utf8(path_bytes).expect("Invalid UTF-8 value");
        offset += len_path;

        //On récupère la longueur de la data
        let len_data : usize = deserialize_u64(&binary, &offset)? as usize;
        offset += 8;

        //On récupère la data
        if binary.len() < offset + len_data{
            return Err(String::from("File size written in binary is incorrect"));
        }
        let mut data : Vec<u8> = Vec::new();
        for i in offset..offset + len_data {
            data.push(binary[i]);
        }

        Ok(Self::new(&path, data))
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

    pub fn serialize(&mut self) -> Vec<u8> {
        //On créé le fichier binaire
        let mut binary_file : Vec<u8> = Vec::new();

        //On ajoute la taille du path au binary
        let len_path : u64 = self.path.len() as u64;
        let len_path : [u8;8] = len_path.to_le_bytes();
        binary_file.extend(len_path); //len_path est moved du coup deallocated

        //On ajoute le path au fichier
        binary_file.extend(self.path.as_bytes());

        //On ajoute ensuite le nombre de fichier dans le binary
        let nb_file : u64 = self.children_file.len() as u64;
        let nb_file : [u8;8] = nb_file.to_le_bytes();
        binary_file.extend(nb_file);

        //On ajoute ensuite chaque fichier serializé au binary
        for file in &mut self.children_file {
            binary_file.extend(file.serialize());
        }

        //On ajoute ensuite le nombre de dossier dans le binary
        let nb_dir : u64 = self.children_directory.len() as u64;
        let nb_dir : [u8;8] = nb_dir.to_le_bytes();
        binary_file.extend(nb_dir);

        //On serialize recursivement chaque dossier
        for dir in &mut self.children_directory {
            binary_file.extend(dir.serialize());
        }

        binary_file
    }

    pub fn deserialize(binary : &Vec<u8>) -> Result<Directory, String>{
        Directory::deserialize_d(binary, 0)
    }

    fn deserialize_d(binary : &Vec<u8>, offset_f : usize) -> Result<Directory, String>{
        let mut offset : usize = offset_f;

        //On récupère la taille du path
        let len_path : usize = deserialize_u64(&binary, &offset)? as usize;
        offset += 8;

        //On récupère le path
        if binary.len() < offset + len_path{
            return Err(String::from("Path sized written in binary is incorrect"));
        }
        let mut path_bytes : Vec<u8> = Vec::new();
        for i in offset..offset + len_path {
            path_bytes.push(binary[i]);
        }
        let path : String = String::from_utf8(path_bytes).expect("Invalid UTF-8 value");
        offset += len_path;

        //On créé l'objet Directory
        let mut dir = Self::new_empty(&path);

        //On lit le nombre de fichier présents
        let nb_files : usize = deserialize_u64(&binary, &offset)? as usize;
        offset += 8;

        //Pour chaque fichier, on le deserialize et on l'ajoute au dossier
        for _ in 0..nb_files{
            let file : File = File::deserialize_f(&binary, offset)?;
            offset += 8 + file.path.len() + 8 + file.data.len();
            dir.children_file.push(file);
        }

        //On lit le nombre de dossier présent
        let nb_dir : usize = deserialize_u64(&binary, &offset)? as usize;
        offset += 8;

        //Pour chaque dossier, on le deserialize et on l'ajoute au dossier
        for _ in 0..nb_dir{
            let sub_dir : Directory = Self::deserialize_d(&binary, offset)?;
            offset += Self::get_offset_dir_deserialization(&sub_dir);
            dir.children_directory.push(sub_dir);
        }

        Ok(dir)
    }

    pub fn get_offset_dir_deserialization(dir : &Directory) -> usize {
        let mut offset : usize = 0;
        offset += 8 + dir.path.len();   //Pour trouver le titre
        offset += 8;                    //Pour le nombre de fichier
        for file in &dir.children_file{
            //Pour la taille du path + le path et pour la taille de la data + la data
            offset += 8 + file.path.len() + 8 + file.data.len();
        }
        offset += 8;                    //Pour le nombre de dossier
        for dir in &dir.children_directory{
            offset += Self::get_offset_dir_deserialization(&dir);
        }

        offset
    }

    pub fn write_binary(&mut self, path : &str){
        let binary = self.serialize();
    
        match fs::write(path, binary){
            Ok(_) => println!("File written to {}", path),
            Err(error) => println!("Couldn' write file : {}", error)
        };
    }

    pub fn read_binary(path : &str) -> Result<Self, String>{
        match fs::read(path) {
            Ok(result) => return Self::deserialize(&result),
            Err(err) => return Err(format!("Couldn't read the file : {}", err))
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

fn deserialize_u64(binary : &Vec<u8>, offset : &usize) -> Result<u64, String> {
    if binary.len() < offset+8{
        return Err(String::from("binary is too small to have a u64 at this offset location"));
    }
    let mut num : [u8;8] = [0;8];
    num.copy_from_slice(&binary[*offset..*offset+8]);
    Ok(u64::from_le_bytes(num))
}



pub fn restore_arch(path : String, dir : Directory) {

}