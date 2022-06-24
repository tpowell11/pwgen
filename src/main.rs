use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::io;
use rand::Rng;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng; 
//use clipboard::{ClipboardProvider,ClipboardContext}; //!NYI

fn main() {
    let choice = vec!['!','@','#','$','%','^','&','*','(',')','[',']','|','~','<','>',',','.'];
    //let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    loop {
        println!("Enter service name: ");
        let mut name = String::new();
        let mut pwd = String::new();
        let mut seed: u32 = 0;
        io::stdin()
            .read_line(&mut name)
            .expect("Read Failiure, exiting");
        for ch in name.chars() {
            seed += ch as u32;
        }
        let mut rng = ChaCha20Rng::seed_from_u64(seed as u64);
        let mut hasher = Sha256::new();
        hasher.input_str(&name.trim_end()); 
        for (i, ch) in hasher.result_str().chars().enumerate(){
            if i % 4 == 0 {
                let ind = rng.gen_range(0..choice.len());
                pwd.push(choice[ind]);
            } else {
                pwd.push(ch);
            }
        }
        println!("Enter desired length (required): ");
        let mut tlen = String::new();
        io::stdin()
            .read_line(&mut tlen)
            .expect("Read Failiure, exiting");
        let tlen_as_int = tlen.trim();
        match tlen_as_int.parse::<usize>() {
            Ok(i) => pwd.truncate(i),
            Err(..) => println!("Password Truncation Failed")
        };
        println!("Service:  {}", name);
        println!("Password: {}", pwd);
        //ctx.set_contents(pwd).unwrap();
        //println!("Password copied to clipboard");
        //println!("--------------");
    } 
}
