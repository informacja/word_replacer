use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main() {
    let mut command_line: std::env::Args = std::env::args();
   
    let file_name;
    let c1;
    let c2;
        
    if(1) // to do: num of args
    {
        println("Example command line: main.exe ../Joe.txt this sth");

        file_name = "./Joe.txt"; 
        c1 = "this";
        c2 = "sth";
        
        let data = "Writing this to a this file.\n";
        fs::write(file_name, data).expect("Unable to write file");
    } 
    else
    {        
        command_line.next().unwrap();                                           // skip the executable file name
        file_name = command_line.next().unwrap();
        c1        = command_line.next().unwrap();
        c2        = command_line.next().unwrap();        
    }

// ----------------------------------------------------------------------------
    // let f = File::open(file_name).expect("Unable to open file");
    // let f = BufReader::new(f);

    // for line in f.lines() {
    //     let line = line.expect("Unable to read line");
    //     println!("Line: {}", line);
    // }

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    // let result = "Hello World! !".replace("!", "?");
    // println!("{}", result); // => "Hello World? ?"
    contents = contents.replace(c1, c2); 

    let mut file = File::create(file_name).expect("create failed");
    file.write_all(contents.as_bytes()).expect("write failed");
//    file.write_all("Hello World".as_bytes()).expect("write failed");

    // let mut data;
    // let a = String::new();
    // let mut f = File::open(file_name).expect("Unable to open file");
    // f.read_to_string(&mut data).expect("Unable to read string");
    // println!("{}", data);



// kolokwium prof. Filipowixz
// reguła mnemotechniczna do opisow systemow

}


// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let mut dat
//     a = String::new();
//     let mut f = File::open("/etc/hosts").expect("Unable to open file");
//     f.read_to_string(&mut data).expect("Unable to read string");
//     println!("{}", data);
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
