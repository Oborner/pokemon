pub mod poke{
    use std::clone::Clone;
    mod eficiencia;
    mod efeitos;
    use crate::pokemon_monstro::poke::eficiencia::eficiencia;
    use crate::pokemon_monstro::poke::efeitos::efeito;
    use rand::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;


    //Structs do pokemon e do ataque
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
        pub statusComeço: StatusComeçoTurno,
        pub statusFinal: StatusFinalTurno,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Attack{
        pub nome: String,
        pub categoria: Categoria,
        pub power: f32,
        pub accuracy: u8,
        pub pp: u8,
        pub tipo: Tipo,
        pub statusComeço: StatusComeçoTurno,
        pub statusFinal: StatusFinalTurno,
    }

    // enums do tipo do pokemon/tipo do ataque e a categoria de cada ataque
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

    #[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
    pub enum Categoria{
        Físico,
        Especial,
    }

    #[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
    pub enum StatusFinalTurno{
        Burn, // 1/8 do hp maximo dano
        Poison, // 1 dano turno
        Curse, // 1/4 do hp maximo dano
        Rooting, // cura 1/16 do hp máximo
        Bound, // dura de 4-5 turnos
        None
    }

    #[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
    pub enum StatusComeçoTurno{
        Freeze, //20% de chance de sair
        Paralysis = 2, //25% de chance de n atacar, duração 2 turnos
        Sleep = 4, //dorme de 1 a 7 turnos
        Confusion, //dura de 2 - 5 turnos
        None
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
                statusComeço: StatusComeçoTurno::None,
                statusFinal: StatusFinalTurno::None
            }
        }
    }

    //Instanciação do ataque
    impl  Attack{
        pub fn novo (nome:String, categoria:Categoria, power:f32,accuracy:u8, pp:u8, tipo:Tipo, statusComeço:StatusComeçoTurno, statusFinal:StatusFinalTurno) -> Self{
            Attack{
                nome,
                categoria,
                power,
                accuracy,
                pp,
                tipo,
                statusComeço,
                statusFinal,
            }
        }
        
    }

    impl FromStr for Tipo {
            type Err = ();

        fn from_str(input: &str) -> Result<Tipo, Self::Err> {
            match input {
                "Normal" => Ok(Tipo::Normal),
                "Fire" => Ok(Tipo::Fire),
                "Water" => Ok(Tipo::Water),
                "Grass" => Ok(Tipo::Grass),
                "Electric" => Ok(Tipo::Electric),
                "Ice" => Ok(Tipo::Ice),
                "Fighting" => Ok(Tipo::Fighting),
                "Poison" => Ok(Tipo::Poison),
                "Ground" => Ok(Tipo::Ground),
                "Flying" => Ok(Tipo::Flying),
                "Psychic" => Ok(Tipo::Psychic),
                "Bug" => Ok(Tipo::Bug),
                "Rock" => Ok(Tipo::Rock),
                "Ghost" => Ok(Tipo::Ghost),
                "Dragon" => Ok(Tipo::Dragon),
                "Dark" => Ok(Tipo::Dark),
                "Steel" => Ok(Tipo::Steel),
                "Fairy" => Ok(Tipo::Fairy),
                _      => Err(()),
            }
        }
    }

    impl FromStr for StatusFinalTurno {
        type Err = ();

    fn from_str(input: &str) -> Result<StatusFinalTurno, Self::Err> {
        match input {
            "Burn"=> Ok(StatusFinalTurno::Burn), 
            "Poison"=> Ok(StatusFinalTurno::Poison), 
            "Curse"=> Ok(StatusFinalTurno::Curse), 
            "Rooting"=> Ok(StatusFinalTurno::Rooting), 
            "Bound"=> Ok(StatusFinalTurno::Bound), 
            "None"=> Ok(StatusFinalTurno::None),
            _      => Err(()),
        }
    }
}
impl FromStr for Categoria {
    type Err = ();

fn from_str(input: &str) -> Result<Categoria, Self::Err> {
    match input {
        "Físico"=> Ok(Categoria::Físico),
        "Especial"=> Ok(Categoria::Especial),
        _      => Err(()),
    }
}
}
impl FromStr for StatusComeçoTurno {
    type Err = ();

fn from_str(input: &str) -> Result<StatusComeçoTurno, Self::Err> {
    match input {
        "Freeze"=> Ok(StatusComeçoTurno::Freeze), 
        "Paralysis" => Ok(Self::Paralysis), 
        "Sleep" => Ok(StatusComeçoTurno::Sleep), 
        "Confusion"=> Ok(StatusComeçoTurno::Confusion), 
        "None"=> Ok(StatusComeçoTurno::None),
        _      => Err(()),
    }
}
}

    //Função para realizar o ataque
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

            println!("{:?}", inimigo.hp);
        }
    }
}