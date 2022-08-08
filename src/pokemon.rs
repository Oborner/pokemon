#[derive(Debug)]
pub struct Pokemon{
    pub nome: String,
    pub hp: u16,
    pub atack: f32,
    pub defense: f32,
    pub sp_atack: f32,
    pub sp_defense: f32,
    pub speed: f32,
    pub attacks: Vec<Attack>
}

#[derive(Debug, Clone)]
pub struct Attack{
    pub name: String,
    pub tipo: Tipo,
    pub power: f32,
}

#[derive(Debug)]
#[derive(PartialEq, Clone)]
pub enum Tipo{
    Físico,
    Especial,
}

pub trait Ataque {
    fn ataca(&self, inimigo: &mut Self, ind:u16);
}

impl Pokemon{
    pub fn novo(nome:String, hp: u16, atack: f32, sp_defense: f32, sp_atack: f32, defense: f32, speed: f32, attacks: Vec<Attack>)-> Self{
        Pokemon{
            nome: nome,
            hp: hp,
            atack: atack,
            sp_defense,
            sp_atack,
            defense: defense,
            speed: speed,
            attacks,
        }
    }
}

impl  Attack{
    pub fn novo (name:String, tipo:Tipo, power:f32) -> Self{
        Attack{
            name,
            tipo,
            power,
        }
    }
    
}

impl Ataque for Pokemon{
    fn ataca(&self, inimigo: &mut Self, i: u16){
        let a:usize = i.into();
        let tipo = &self.attacks[a].tipo;
        let dano = &self.attacks[a].power;
        let cem:f32 = 100.000;
        if tipo == &Tipo::Físico{
            if inimigo.hp <= (dano*(cem/(cem + inimigo.defense))) as u16 {
                inimigo.hp = 0;
            } else {
                inimigo.hp -= (dano*(cem/(cem + inimigo.defense))) as u16;
            }
        }else{
            if inimigo.hp <= (dano*(cem/(cem + inimigo.sp_defense))) as u16 {
                inimigo.hp = 0;
            } else {
                inimigo.hp -= (dano*(cem/(cem + inimigo.sp_defense))) as u16;
            }
        }
        println!("{:?}", inimigo.hp);

    }
}