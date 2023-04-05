use std::fs::File;

mod read_file;
fn main() {
    let file = read_file::file::new("C:\\Users\\toney\\Documents\\GitHub\\file-splitter\\src\\text.txt", 20).unwrap();
    
    println!("{:?}", file.get_bytes());
}

    
