pub mod poke{
    use std::clone::Clone;
    mod eficiencia;
    use crate::pokemon_monstro::poke::eficiencia::eficiencia;
    use rand::prelude::*;
    use serde::{Deserialize, Serialize};


    //Structs do pokemon 
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Pokemon{
        pub nome: String,
        pub hp_max: u16,
        pub hp: u16,
        pub attack: f32,
        pub defense: f32,
        pub sp_attack: f32,
        pub sp_defense: f32,
        pub speed: f32,
        pub evasion: u8,
        pub p_tipo: Tipo,
        pub attacks: Vec<Attack>,
    }

    //Struct do ataque
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Attack{
        pub nome: String,
        pub categoria: Categoria,
        pub power: f32,
        pub accuracy: u8,
        pub pp: u8,
        pub tipo: Tipo,
    }

    // enum do tipo do pokemon/tipo do ataque 
    #[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
    pub enum Tipo{
        Normal,
        Fire,
        Water,
        Grass,
        Electric,
        Ice,
        Fighting,
        Poison,
        Ground,
        Flying,
        Psychic,
        Bug,
        Rock,
        Ghost,
        Dragon,
        Dark,
        Steel,
        Fairy,
    }

    // Enum com a categoria do ataque
    #[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
    pub enum Categoria{
        Físico,
        Especial,
    }


    //Trait para implementar o ataque
    pub trait Ataque {
        fn ataca(atacante: &mut Self, inimigo: &mut Self, ind:usize);
    }

    // Instanciação do pokemon
    impl Pokemon{
        pub fn novo(nome:String, hp: u16, hp_max:u16, attack: f32, defense: f32, sp_attack: f32, sp_defense: f32, speed: f32, evasion: u8, p_tipo:Tipo, attacks: Vec<Attack>)-> Self{
            Pokemon{
                nome,
                hp,
                hp_max,
                attack,
                sp_defense,
                sp_attack,
                defense,
                speed,
                evasion,
                p_tipo,
                attacks,
            }
        }
    }

    //Instanciação do ataque
    impl  Attack{
        pub fn novo (nome:String, categoria:Categoria, power:f32,accuracy:u8, pp:u8, tipo:Tipo) -> Self{
            Attack{
                nome,
                categoria,
                power,
                accuracy,
                pp,
                tipo,
            }
        }
        
    }

    //Função para calcular o dano do ataque
    impl Ataque for Pokemon{
        fn ataca(atacante: &mut Self, inimigo: &mut Self, i: usize){

            let ataque_categoria = atacante.attacks[i].categoria.clone();
            let poder = atacante.attacks[i].power.clone();
            let tipo_ataque = atacante.attacks[i].tipo.clone();
            let tipo_inimigo = inimigo.p_tipo.clone();
            let p_ataque;
            let p_defesa;
            let random:f32 = rand::thread_rng().gen_range(85.0..101.0)/100.0;


            if ataque_categoria == Categoria::Físico {
                p_ataque = atacante.attack.clone();
                p_defesa = inimigo.defense.clone();
            } else {
                p_ataque = atacante.sp_attack.clone();
                p_defesa = inimigo.sp_defense.clone();
            }

            //Função para calcular a vantagem do ataque
            let modificador = eficiencia(tipo_ataque, tipo_inimigo); 
            
            //Função para calcular o dano
            let mut damage = (((((2.0*75.0/5.0)+2.0)*poder*(p_ataque/p_defesa))/50.0)+2.0)*random*modificador;

            //Arredondamento do dano para cima e conversão para u16
            damage = damage.ceil();
            let damage_int = damage as u16;

            //Dano sendo aplicado no hp do inimigo
            if damage_int>inimigo.hp{
                inimigo.hp = 0;
            } else {
                inimigo.hp -= damage_int;
            }

            //Redução do pp(pontos de ação) do ataque
            atacante.attacks[i].pp -= 1;

            println!("O {} causa {} de dano", atacante.nome, damage_int);
            println!("O {} fica com {} de vida",inimigo.nome, inimigo.hp);
        }
    }
}