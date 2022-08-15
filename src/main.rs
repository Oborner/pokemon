mod pokemon;
use pokemon::*;
use serde::{Serialize, Deserialize};
use mongodb::*;
use futures::executor::block_on;

async fn conectar()-> Client{
    Client::with_uri_str("mongodb://localhost:27017").await?
}
fn main() {
    let client = conectar();
    block_on(future);

    let db = client.database("Pokemon").collection("Pokemon");

    println!("Funfou");
    let mut ataques:Vec<Attack> = [].to_vec();
    let ataque1 = Attack::novo("Folha".to_string(), Categoria::Físico, 50.0, 5, Tipo::Grass);
    let ataque2 = Attack::novo("Folha2".to_string(), Categoria::Físico, 50.0, 5, Tipo::Grass);

    ataques.push(ataque1);
    ataques.push(ataque2);


    let mut gab = Pokemon::novo("bulba".to_string(), 50, 50, 64.0, 64.0, 50.0, 50.0, 100.0,Tipo::Grass, ataques.clone());
    let mut gab2 = Pokemon::novo("bulba".to_string(), 50, 50, 64.0, 64.0, 50.0, 50.0, 100.0,Tipo::Grass, ataques);

    pokemon::Pokemon::ataca(&mut gab, &mut gab2, 1);

    let serializado = serde_json::to_string(&gab).unwrap();
    println!("Serializado: {}", serializado);
}
