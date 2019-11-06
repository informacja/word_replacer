use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
   
    let file_name : std::string::String;
    let c1        : std::string::String;
    let c2        : std::string::String;
        
    if(true) // to do: num of args
    {
        println!("Example command line: main.exe ../Joe.txt this sth");

        file_name = "./Joe.txt".to_string(); 
        c1 = "this".to_string();
        c2 = "sth".to_string();
        
        let data = "Writing this to a this file.\n";
        fs::write(file_name.to_string(), data).expect("Unable to write file");
    } 
    else
    {        
        let mut command_line: std::env::Args = std::env::args();
        
        command_line.next().unwrap();                                           // skip the executable file name
        file_name = command_line.next().unwrap();
        c1        = command_line.next().unwrap();
        c2        = command_line.next().unwrap();        
    }
                                                                                // Mario was here
































// ----------------------------------------------------------------------------

    let mut in_file = File::open(file_name.to_string()).unwrap();
    let mut contents = String::new();
    in_file.read_to_string(&mut contents).unwrap();
    // print!("Readed: {}", contents);

    contents = contents.replace(&c1.to_string(), &c2.to_string()); 

    let mut out_file = File::create(file_name.to_string()).expect("create failed");
    // print!("Saved : {}", contents);
    out_file.write_all(contents.as_bytes()).expect("write failed");

}














// kolokwium prof. Filipowixz
// reguła mnemotechniczna do opisow systemow




// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let mut dat
//     a = String::new();
//     let mut f = File::open("/etc/hosts").expect("Unable to open file");
//     f.read_to_string(&mut data).expect("Unable to read string");
//     println!("{}", data);
// }
    // let f = File::open(file_name).expect("Unable to open file");
    // let f = BufReader::new(f);

    // for line in f.lines() {
    //     let line = line.expect("Unable to read line");
    //     println!("Line: {}", line);
    // }

// use std::fs::File;
// use std::io::{BufRead, BufReader};

// fn main() {
//     let f = File::open("/etc/hosts").expect("Unable to open file");
//     let f = BufReader::new(f);

//     for line in f.lines() {
//         let line = line.expect("Unable to read line");
//         println!("Line: {}", line);
//     }
// }


    // let result = "Hello World! !".replace("!", "?");
    // println!("{}", result); // => "Hello World? ?"
    //    file.write_all("Hello World".as_bytes()).expect("write failed");

    // let mut data;
    // let a = String::new();
    // let mut f = File::open(file_name).expect("Unable to open file");
    // f.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);

