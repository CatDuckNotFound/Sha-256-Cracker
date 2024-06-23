use std::env;
use std::io::{BufRead,BufReader};
use std::fs::File;
use std::process::exit;
use sha2::{Sha256,  Digest};
//importing Dependencies


fn main() {

    //Check if argument is provied or it fails
    let args: Vec<String> = env::args().collect();
    if args.len()  !=  2{
        println!("\n\nInvalid Arguments");
        println!("Try \"Cargo run SHA256-hash\"");
        exit(1);
    }


let wanted_hash = &args[1];
let password_files = ["src/xaa","src/xab","src/xac","src/xad","src/xae"];
    let mut attempt = 1;

for password_file in password_files {
    println!("\n\nCraking Hash {} ", wanted_hash);
    let password_list = File::open(password_file).unwrap();
    let reader = BufReader::new(password_list);

    for line in reader.lines() {
        let line = line.unwrap();
        let password = line.trim().to_owned().into_bytes();
        let password_hash = format!("{:x}", Sha256::digest(&password));
        println!("[{}] {}  = {} ", attempt, std::str::from_utf8(&password).unwrap(), password_hash);

        if &password_hash == wanted_hash {
            println!("Password hash found after {} attempt {} changed to {}", attempt, std::str::from_utf8(&password).unwrap(), password_hash);
            exit(0);
        }
        attempt += 1;
    }
}
    println!("Password hash not found");





}
