use super::Tipo;

pub fn eficiencia (t1:Tipo, t2:Tipo) -> f32{
    let mut eficiencia:f32 = 1.0;

    match t1 {
        Tipo::Normal => if t2==Tipo::Rock || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Ghost{eficiencia=0.0},
        Tipo::Fire => if t2==Tipo::Fire || t2==Tipo::Water || t2==Tipo::Rock || t2==Tipo::Dragon{eficiencia=0.5} else if t2==Tipo::Grass || t2==Tipo::Ice || t2==Tipo::Bug || t2==Tipo::Steel{eficiencia=2.0},
        Tipo::Water => if t2==Tipo::Water || t2==Tipo::Grass || t2==Tipo::Dragon{eficiencia=0.5} else if t2==Tipo::Fire || t2==Tipo::Ground || t2==Tipo::Rock{eficiencia=2.0},
        Tipo::Grass => if t2==Tipo::Fire || t2==Tipo::Grass || t2==Tipo::Poison || t2==Tipo::Flying || t2==Tipo::Bug || t2==Tipo::Dragon || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Water || t2==Tipo::Ground || t2==Tipo::Rock{eficiencia=2.0},
        Tipo::Electric => if t2==Tipo::Grass || t2==Tipo::Electric || t2==Tipo::Dragon{eficiencia=0.5} else if t2==Tipo::Water || t2==Tipo::Flying{eficiencia=2.0} else if t2==Tipo::Ground{eficiencia=0.0},
        Tipo::Ice => if t2==Tipo::Fire || t2==Tipo::Water || t2==Tipo::Ice || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Grass || t2==Tipo::Ground || t2==Tipo::Flying || t2==Tipo::Dragon{eficiencia=2.0},
        Tipo::Fighting => if t2==Tipo::Poison || t2==Tipo::Flying || t2==Tipo::Psychic || t2==Tipo::Bug || t2==Tipo::Fairy{eficiencia=0.5} else if t2==Tipo::Normal || t2==Tipo::Ice || t2==Tipo::Rock || t2==Tipo::Dark || t2==Tipo::Steel{eficiencia=2.0} else if t2==Tipo::Ghost{eficiencia=0.0},
        Tipo::Poison => if t2==Tipo::Poison || t2==Tipo::Ground || t2==Tipo::Rock || t2==Tipo::Ghost{eficiencia=0.5} else if t2==Tipo::Grass || t2==Tipo::Fairy{eficiencia=2.0} else if t2==Tipo::Steel{eficiencia=0.0},
        Tipo::Ground => if t2==Tipo::Electric || t2==Tipo::Bug{eficiencia=0.5} else if t2==Tipo::Fire || t2==Tipo::Electric || t2==Tipo::Poison || t2==Tipo::Rock || t2==Tipo::Steel{eficiencia=2.0} else if t2==Tipo::Flying{eficiencia=0.0},
        Tipo::Flying => if t2==Tipo::Electric || t2==Tipo::Rock || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Grass || t2==Tipo::Fighting || t2==Tipo::Bug{eficiencia=2.0},
        Tipo::Psychic => if t2==Tipo::Psychic || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Fighting || t2==Tipo::Poison{eficiencia=2.0} else if t2==Tipo::Dark{eficiencia=0.0},
        Tipo::Bug => if t2==Tipo::Fire || t2==Tipo::Fighting || t2==Tipo::Poison || t2==Tipo::Flying || t2==Tipo::Dragon || t2==Tipo::Steel || t2==Tipo::Fairy{eficiencia=0.5} else if t2==Tipo::Grass || t2==Tipo::Psychic || t2==Tipo::Dark{eficiencia=2.0},
        Tipo::Rock => if t2==Tipo::Fighting || t2==Tipo::Ground || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Fire || t2==Tipo::Ice || t2==Tipo::Flying || t2==Tipo::Bug{eficiencia=2.0},
        Tipo::Ghost => if t2==Tipo::Dark{eficiencia=0.5} else if t2==Tipo::Psychic || t2==Tipo::Ghost{eficiencia=2.0} else if t2==Tipo::Normal{eficiencia=0.0},
        Tipo::Dragon => if t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Ghost{eficiencia=2.0} else if t2==Tipo::Fairy{eficiencia=0.0},
        Tipo::Dark => if t2==Tipo::Fighting || t2==Tipo::Dark || t2==Tipo::Fairy{eficiencia=0.5} else if t2==Tipo::Psychic || t2==Tipo::Ghost{eficiencia=2.0},
        Tipo::Steel => if t2==Tipo::Fire || t2==Tipo::Water || t2==Tipo::Electric || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Ice || t2==Tipo::Rock || t2==Tipo::Fairy {eficiencia=2.0},
        Tipo::Fairy => if t2==Tipo::Fire || t2==Tipo::Poison || t2==Tipo::Steel{eficiencia=0.5} else if t2==Tipo::Fighting || t2==Tipo::Dragon || t2==Tipo::Dark{eficiencia=2.0},
    }

    eficiencia
}