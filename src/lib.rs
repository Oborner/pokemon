

pub mod inteiro {

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub fn lê_int(msg: &str) -> u32 {
    println!("{}", msg);

    let mut inteiro = String::new();

    io::stdin()
        .read_line(&mut inteiro)
        .expect("Falha na leitura da linha");
        
    
    inteiro.trim().parse().unwrap()
}

pub fn lê_ok<T: FromStr>() -> T
    where <T as FromStr>::Err: Debug
{

    let mut número = String::new();

    io::stdin()
        .read_line(&mut número)
        .expect("Falha na leitura da linha");
        
    
    número.trim().parse().unwrap()
}

pub fn lê<T: FromStr>(msg: &str) -> Option<T> {
    let mut número = String::new();

    println!("{}", msg);
    io::stdin()
        .read_line(&mut número)
        .expect("Falha na leitura da linha");
        
    match número.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
} 

}



