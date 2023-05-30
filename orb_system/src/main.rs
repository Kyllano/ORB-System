mod file_management;
use file_management::Directory;
use file_management::File;



fn main() {
    //_test_arch();
    let mut root : Directory = Directory::new_empty("./test");
    root.explore_hierarchy();
    //root.print_arch();
}






fn _test_arch() {
    let mut root = Directory::new_empty("root");
    root.add_directory(Directory::new_empty("subfolder"));
    root.add_file(File::new_empty("lol"));
    root.children_file[0].read_data_from_file("../test/README.md");
    root.add_file(File::new_empty("prout"));
    root.children_directory[0].add_file(File::new_empty("homework"));

    root.print_arch();
    println!("Contenu de lol :");
    for file in root.children_file{
        if file.path == "lol"{
            file.print_file_content_as_str();
            file.print_file_content_as_str();
        }
    }
}