mod lib;
use lib::{inteiro::lê_ok};
mod pokemon_monstro;
use crate::pokemon_monstro::poke::{*, self};
use std::{fs::{File}, io::{Read}};
use colored::*;

    fn main()  -> Result<(), serde_json::Error> {
        //Abertura do arquivo contendo os pokemons
        let mut arquivo = File::open("pokemons_file/pokemons.json")
        .expect("Erro");
    
        //Leitura do arquivo
        let mut texto: String = String::new();
        arquivo.read_to_string(&mut texto).
        expect("erro");
    
        //Transformando os dados do arquivo em um vetor de structs pokemon
        let list_pokemon: Vec<Pokemon> = 
        serde_json::from_str(&texto)?;
        
        //Aqui é listado os pokemons e os jogadores escolhem seus pokemons
        let mut count = 0;
        for i in &list_pokemon{
            println!("{} - {}", count, i.nome);
            count +=1;
        }

        println!("Escolha o numero do pokemon do jogador 1");
        let num1:usize = lê_ok();

        println!("Escolha o numero do pokemon do jogador 2");
        let num2:usize = lê_ok();

        //Listado os pokemons escolhidos
        println!("Player 1 escolheu: {:?}", list_pokemon[num1].nome);
        println!("Player 2 escolheu: {:?}", list_pokemon[num2].nome);

        //Aqui os pokemons são clonados da lista e colocados nas variáveis
        let mut poke1 = list_pokemon[num1].clone();
        let mut poke2 = list_pokemon[num2].clone();
        let mut round = 1;

        //Aqui se inicia a luta
        'luta: loop{
            println!();
            println!("================================================");
            println!("{} {}", "Round: ".cyan(), round.to_string().cyan());
            println!("================================================");
            println!();
            println!("Player 1 escolha o ataque: ");
            println!("{:?}", poke1.attacks);
            count = 0;

            //Aqui são listados os ataque do primeiro pokemon
            //O player 1 escolhe seu ataque
            for i in &poke1.attacks{
                println!("{} - {}", count, i.nome);
                count +=1;
            }
            let num_attack1 = lê_ok();
            println!();

            //Aqui são listados os ataque do segundo pokemon
            //O player 2 escolhe seu ataque
            println!("Player 2 escolha o ataque: ");
            println!("{:?}", poke2.attacks);
            count = 0;
            for i in &poke2.attacks{
                println!("{} - {}", count, i.nome);
                count +=1;
            }
            let num_attack2 = lê_ok();

            //Aqui é verificada a velocidade dos pokemons para definir que ataca primeiro
            if poke1.speed>poke2.speed{
                //O primeiro pokemon ataca
                println!("================================================");
                println!("{} usa {:?}\n", poke1.nome , poke1.attacks[num_attack1]);
                poke::Pokemon::ataca(&mut poke1, &mut poke2, num_attack1);
                println!("================================================");
                //Aqui é definido o pokemon ganhador
                if poke2.hp == 0 {
                    println!("{}", "O player 1 ganhou".red());
                    break 'luta;
                }
                //O segundo pokemon ataca
                println!("{} usa {:?}\n", poke2.nome , poke2.attacks[num_attack2]);
                poke::Pokemon::ataca(&mut poke2, &mut poke1, num_attack2);
                println!("================================================");
                if poke1.hp == 0 {
                    println!("{}", "O player 2 ganhou".red());
                    break 'luta;
                }
                
            }else if poke1.speed<poke2.speed{
                println!("================================================");
                println!("{} usa {:?}\n", poke2.nome , poke2.attacks[num_attack2]);
                poke::Pokemon::ataca(&mut poke2, &mut poke1, num_attack2);
                println!("================================================");
                if poke1.hp == 0 {
                    println!("{}", "O player 2 ganhou".red());
                    break 'luta;
                }
                
                println!("{} usa {:?}\n", poke1.nome , poke1.attacks[num_attack1]);
                poke::Pokemon::ataca(&mut poke1, &mut poke2, num_attack1);
                println!("================================================");
                if poke2.hp == 0 {
                    println!("{}", "O player 1 ganhou".red());
                    break 'luta;
                }
            }

            
            round +=1;
        }

        Ok(())
    }








