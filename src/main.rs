mod pokemon;
use pokemon::*;


fn main() {
    println!("Funfou");

    let ataque1 = Attack::novo("Poro Venenoso".to_string(), Tipo::Especial, 40 as f32);
    let ataque2 = Attack::novo("Chicote de Vinha".to_string(), Tipo::Físico, 45 as f32);
    let ataque3 = Attack::novo("Faca de Dois Gumes".to_string(), Tipo::Físico, 120 as f32);
    let ataque4 = Attack::novo("Raio Solar".to_string(), Tipo::Especial, 120 as f32);

    let mut ataques = Vec::new();
    ataques.push(ataque1);
    ataques.push(ataque2);
    ataques.push(ataque3);
    ataques.push(ataque4);

    let player = Pokemon::novo("Bulbassauro".to_string(), 45 ,49 as f32,65 as f32,65 as f32, 49 as f32, 45 as f32, ataques.clone());

    let mut player2 = Pokemon::novo("Bulbassauro".to_string(), 45 ,49 as f32,65 as f32,65 as f32, 49 as f32, 45 as f32, ataques.clone());

    let dano = &player.attacks[2].tipo;
    println!("{:?}\n", dano);
    println!("{:?}\n", player);
    println!("{:?}\n", player.attacks[2]);

    let poder = player.attacks[0].power;
    let defesa = player.defense;
    let cem:f32 = 100.000;

    let calculo = player.attacks[0].power*(cem/(cem + player.defense));
    println!("O dano é {},\nO poder é: {}\n A defesa é: {}", calculo, poder, defesa);
    println!("O dano int é {}", calculo.ceil() as u16);
    println!("{:?}",player2.hp);
    player.ataca(&mut player2, 0);
    player.ataca(&mut player2, 1);

}

