use std::clone::Clone;
mod eficiencia;
use crate::pokemon::eficiencia::eficiencia;
use rand::prelude::*;
use serde::{Deserialize, Serialize};


pub struct Jogador{
    pub nome: String,
    pub pokemons: Vec<Pokemon>,
    pub bag: Vec<Itens>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Pokemon{
    pub nome: String,
    pub hp_max: u16,
    pub hp: u16,
    pub attack: f32,
    pub defense: f32,
    pub sp_attack: f32,
    pub sp_defense: f32,
    pub speed: f32,
    pub p_tipo: Tipo,
    pub attacks: Vec<Attack>
}

pub struct Itens{
    pub nome: String,
    pub quant: u8,
    pub cura: u8,
}

#[derive(Debug, Clone, Serialize)]
pub struct Attack{
    pub nome: String,
    pub categoria: Categoria,
    pub power: f32,
    pub pp: u8,
    pub tipo: Tipo,
}


#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, Serialize)]
pub enum Categoria{
    Físico,
    Especial,
}

pub trait Ataque {
    fn ataca(atacante: &mut Self, inimigo: &mut Self, ind:usize);
}

impl Jogador {
    pub fn novo (nome:String, pokemons:Vec<Pokemon>, bag:Vec<Itens>)-> Self{
        Jogador{
            nome,
            pokemons,
            bag,
        }
    }
}

impl Pokemon{
    pub fn novo(nome:String, hp: u16, hp_max:u16, attack: f32, defense: f32, sp_attack: f32, sp_defense: f32, speed: f32, p_tipo:Tipo, attacks: Vec<Attack>)-> Self{
        Pokemon{
            nome,
            hp,
            hp_max,
            attack,
            sp_defense,
            sp_attack,
            defense,
            speed,
            p_tipo,
            attacks,
        }
    }
}

impl  Attack{
    pub fn novo (nome:String, categoria:Categoria, power:f32, pp:u8, tipo:Tipo) -> Self{
        Attack{
            nome,
            categoria,
            power,
            pp,
            tipo
        }
    }
    
}

impl Itens{
    pub fn novo (nome:String, quant:u8, cura:u8)-> Self{
        Itens{
            nome,
            quant,
            cura
        }
    }
}


impl Ataque for Pokemon{
    fn ataca(atacante: &mut Self, inimigo: &mut Self, i: usize){
        let ataque_categoria = atacante.attacks[i].categoria.clone();
        let poder = atacante.attacks[i].power.clone();
        let tipo_ataque = atacante.attacks[i].tipo.clone();
        let tipo_inimigo;
        let p_ataque;
        let p_defesa;
        let random:f32 = rand::thread_rng().gen_range(85.0..101.0)/100.0;

        tipo_inimigo = inimigo.p_tipo.clone();

        if ataque_categoria == Categoria::Físico {
            p_ataque = atacante.attack.clone();
            p_defesa = inimigo.defense.clone();
        } else {
            p_ataque = atacante.sp_attack.clone();
            p_defesa = inimigo.sp_defense.clone();
        }

        let modificador = eficiencia(tipo_ataque, tipo_inimigo);
        let mut damage = (((((2.0*75.0/5.0)+2.0)*poder*(p_ataque/p_defesa))/50.0)+2.0)*random*modificador;

        damage = damage.ceil();
        let damage_int = damage as u16;

        if damage_int>inimigo.hp{
            inimigo.hp = 0;
        } else {
            inimigo.hp -= damage_int;
        }

        atacante.attacks[i].pp -= 1;

        println!("{:?}", inimigo.hp);
    }
}