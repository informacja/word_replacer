use std::fs;

fn main() {
    println!("Hello, world!");

    use std::fs::OpenOptions;

// let  file = OpenOptions::new()
//     .read(true)
//     .write(true)
//     .open("foo.txt");

    // file.write("sth");

//  let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
    // println!("{}", data);

     let data = "Some data!";
    fs::write(".", data).expect("Unable to write file");


}
