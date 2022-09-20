pub mod inteiro {

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

pub fn lê_ok<T: FromStr>() -> T
    where <T as FromStr>::Err: Debug
{

    let mut número = String::new();

    io::stdin()
        .read_line(&mut número)
        .expect("Falha na leitura da linha");
        
    
    número.trim().parse().unwrap()
}


}



