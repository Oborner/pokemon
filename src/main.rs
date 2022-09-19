mod lib;
use lib::{inteiro::lê_ok};
mod pokemon_monstro;
use crate::pokemon_monstro::poke::{*, self};
use serde_json::Error;
use std::{fs::{File, self}, io::{Read, Write}, str::FromStr};
use colored::*;

    fn main()  -> Result<(), serde_json::Error> {
        let mut arquivo = File::open("pokemons_file/pokemons.json")
        .expect("Erro");
    
        let mut texto: String = String::new();
        arquivo.read_to_string(&mut texto).
        expect("erro");
    
        let list_pokemon: Vec<Pokemon> = 
        serde_json::from_str(&texto)?;
        
        let mut count = 0;
        for i in &list_pokemon{
            println!("{} - {}", count, i.nome);
            count +=1;
        }

        println!("Escolha o numero do pokemon do jogador 1");
        let num1:usize = lê_ok();

        println!("Escolha o numero do pokemon do jogador 2");
        let num2:usize = lê_ok();

        println!("{:?}", list_pokemon[num1].nome);
        println!("{:?}", list_pokemon[num2].nome);

        let mut poke1 = list_pokemon[num1].clone();
        let mut poke2 = list_pokemon[num2].clone();
        let mut round = 1;

        loop{
            println!();
            println!("================================================");
            println!("{} {}", "Round: ".cyan(), round.to_string().cyan());
            println!("================================================");
            println!();
            println!("Player 1 escolha o ataque: ");
            println!("{:?}", poke1.attacks);
            count = 0;
            for i in &poke1.attacks{
                println!("{} - {}", count, i.nome);
                count +=1;
            }
            let num_attack1 = lê_ok();
            println!();

            println!("Player 2 escolha o ataque: ");
            println!("{:?}", poke2.attacks);
            count = 0;
            for i in &poke2.attacks{
                println!("{} - {}", count, i.nome);
                count +=1;
            }
            let num_attack2 = lê_ok();
            if poke1.speed>poke2.speed{
                println!("================================================");
                println!("{} usa {:?}\n", poke1.nome , poke1.attacks[num_attack1]);
                poke::Pokemon::ataca(&mut poke1, &mut poke2, num_attack1);
                println!("================================================");
                println!("{} usa {:?}\n", poke2.nome , poke2.attacks[num_attack2]);
                poke::Pokemon::ataca(&mut poke2, &mut poke1, num_attack2);
                println!("================================================");
            }
            if poke1.speed<poke2.speed{
                println!("================================================");
                println!("{} usa {:?}\n", poke2.nome , poke2.attacks[num_attack2]);
                poke::Pokemon::ataca(&mut poke2, &mut poke1, num_attack2);
                println!("================================================");
                println!("{} usa {:?}\n", poke1.nome , poke1.attacks[num_attack1]);
                poke::Pokemon::ataca(&mut poke1, &mut poke2, num_attack1);
                println!("================================================");
            }

            if poke1.hp == 0 {
                println!("{}", "O player 2 ganhou".red());
                break;
            }else if poke2.hp == 0 {
                println!("{}", "O player 1 ganhou".red());
                break;
            }
            round +=1;
        }

        Ok(())
    }



fn adicionar_pokemon()-> Result<(), serde_json::Error>{

    let mut arquivo = File::open("pokemons_file/pokemons.json")
    .expect("Erro");

    let mut ctrl:i32 = 0;

    let mut nomeA1:String;
    let mut cateA1:String;
    let mut poweA1:f32;
    let mut accuA1: u8;
    let mut ppA1: u8;
    let mut tipoA1:String;
    let mut stCoA1:String;
    let mut stFiA1:String;
    let mut nomeA2:String;
    let mut cateA2:String;
    let mut poweA2:f32;
    let mut accuA2: u8;
    let mut ppA2: u8;
    let mut tipoA2:String;
    let mut stCoA2:String;
    let mut stFiA2:String;
    let mut nomeA3:String;
    let mut cateA3:String;
    let mut poweA3:f32;
    let mut accuA3: u8;
    let mut ppA3: u8;
    let mut tipoA3:String;
    let mut stCoA3:String;
    let mut stFiA3:String;
    let mut nomeA4:String;
    let mut cateA4:String;
    let mut poweA4:f32;
    let mut accuA4: u8;
    let mut ppA4: u8;
    let mut tipoA4:String;
    let mut stCoA4:String;
    let mut stFiA4:String;

    let mut ataque1;
    let mut ataque2;
    let mut ataque3;
    let mut ataque4;

    let mut nomeP:String;
    let mut hpP:u16;
    let mut ataquef:f32;
    let mut defesaf:f32;
    let mut ataques:f32;
    let mut defesas:f32;
    let mut speed:f32;
    let mut evasion:u8;
    let mut p_tipo:String;
    let mut j;
    
    let mut vecP = Vec::new();

    while ctrl == 0 {
        println!("Nome do 1 ataque");
        nomeA1 = lê_ok();
        println!("Categoria");
        cateA1 = lê_ok();
        println!("power");
        poweA1 = lê_ok();
        println!("accuracy");
        accuA1 = lê_ok();
        println!("pp");
        ppA1 = lê_ok();
        println!("tipo");
        tipoA1 = lê_ok();
        println!("statusComeço");
        stCoA1 = lê_ok();
        println!("StatusFinal");
        stFiA1 = lê_ok();
        ataque1 = Attack::novo(nomeA1 as String, Categoria::from_str(&cateA1).unwrap(), poweA1 as f32, accuA1 as u8, ppA1 as u8, Tipo::from_str(&tipoA1).unwrap(), StatusComeçoTurno::from_str(&stCoA1).unwrap(),  StatusFinalTurno::from_str(&stFiA1).unwrap());

        println!("Nome do 2 ataque");
        nomeA2 = lê_ok();
        println!("Categoria");
        cateA2 = lê_ok();
        println!("power");
        poweA2 = lê_ok();
        println!("accuracy");
        accuA2 = lê_ok();
        println!("pp");
        ppA2 = lê_ok();
        println!("tipo");
        tipoA2 = lê_ok();
        println!("statusComeço");
        stCoA2 = lê_ok();
        println!("StatusFinal");
        stFiA2 = lê_ok();
        ataque2 = Attack::novo(nomeA2 as String, Categoria::from_str(&cateA2).unwrap(), poweA2 as f32, accuA2 as u8, ppA2 as u8, Tipo::from_str(&tipoA2).unwrap(), StatusComeçoTurno::from_str(&stCoA2).unwrap(),  StatusFinalTurno::from_str(&stFiA2).unwrap());
        
        println!("Nome do 3 ataque");
        nomeA3 = lê_ok();
        println!("Categoria");
        cateA3 = lê_ok();
        println!("power");
        poweA3 = lê_ok();
        println!("accuracy");
        accuA3 = lê_ok();
        println!("pp");
        ppA3 = lê_ok();
        println!("tipo");
        tipoA3 = lê_ok();
        println!("statusComeço");
        stCoA3 = lê_ok();
        println!("StatusFinal");
        stFiA3 = lê_ok();
        ataque3 = Attack::novo(nomeA3 as String, Categoria::from_str(&cateA3).unwrap(), poweA3 as f32, accuA3 as u8, ppA3 as u8, Tipo::from_str(&tipoA3).unwrap(), StatusComeçoTurno::from_str(&stCoA3).unwrap(),  StatusFinalTurno::from_str(&stFiA3).unwrap());

        println!("Nome do 4 ataque");
        nomeA4 = lê_ok();
        println!("Categoria");
        cateA4 = lê_ok();
        println!("power");
        poweA4 = lê_ok();
        println!("accuracy");
        accuA4 = lê_ok();
        println!("pp");
        ppA4 = lê_ok();
        println!("tipo");
        tipoA4 = lê_ok();
        println!("statusComeço");
        stCoA4 = lê_ok();
        println!("StatusFinal");
        stFiA4 = lê_ok();
        ataque4 = Attack::novo(nomeA4 as String, Categoria::from_str(&cateA4).unwrap(), poweA4 as f32, accuA4 as u8, ppA4 as u8, Tipo::from_str(&tipoA4).unwrap(), StatusComeçoTurno::from_str(&stCoA4).unwrap(),  StatusFinalTurno::from_str(&stFiA4).unwrap());

        let mut ataquesVec:Vec<Attack> = [].to_vec();
        ataquesVec.push(ataque1);
        ataquesVec.push(ataque2);
        ataquesVec.push(ataque3);
        ataquesVec.push(ataque4);

        println!("Nome pokemon");
        nomeP = lê_ok();
        println!("hp pokemon");
        hpP = lê_ok();
        println!("ataquef pokemon");
        ataquef = lê_ok();
        println!("defesaf pokemon");
        defesaf = lê_ok();
        println!("ataques pokemon");
        ataques = lê_ok();
        println!("defesas pokemon");
        defesas = lê_ok();
        println!("speed pokemon");
        speed = lê_ok();
        println!("evasion pokemon");
        evasion = lê_ok();
        println!("p_tipo pokemon");
        p_tipo = lê_ok();

        let pokemon_j = Pokemon::novo(nomeP as String, hpP as u16, hpP as u16, ataquef as f32, defesaf as f32, ataques as f32, defesas as f32, speed as f32, evasion as u8, Tipo::from_str(&p_tipo).unwrap(), ataquesVec);


        vecP.push(pokemon_j);
        println!("sair 99");
        ctrl = lê_ok();
    }

    j = serde_json::to_string_pretty(&vecP).unwrap(); 
    fs::write("pokemons_file/pokemons.json", j);


    Ok(())

}







