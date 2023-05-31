#![allow(dead_code,unused_imports)]

mod file_management;
use file_management::Directory;
use file_management::File;



fn main() {
    //_test_arch();
    let mut root : Directory = Directory::new_empty("../test");
    root.explore_hierarchy();
    
    let prout = root.serialize();
    let hiiii : Directory = Directory::deserialize(&prout).expect("problem :3");
    println!("{}", hiiii.path);
    hiiii.print_arch();
    for file in &hiiii.children_file{
        file.print_file_content_as_str();
    }

    //let prout = root.children_file[0].serialize();
    //let hiiii : File = File::deserialize(&prout).expect("problem :3");
    //println!("{}", hiiii.path);
    //hiiii.print_file_content_as_str();
}