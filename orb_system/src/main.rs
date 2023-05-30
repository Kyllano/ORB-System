mod file_management;
use file_management::Directory;
use file_management::File;



fn main() {
    //_test_arch();
    let mut root : Directory = Directory::new_empty("../test");
    root.explore_hierarchy();
    
    for file in &mut root.children_file {
        println!("Content of {}", file.path);
        file.read_data_from_file();
        file.print_file_content_as_str();
    }
}