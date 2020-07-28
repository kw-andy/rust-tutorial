// 10 -enum 

enum Direction {

    Haut,
    Bas,
    Gauche,
    Droite


}

fn main() {

    let joueur_direction:Direction = Direction::Bas;
    let toto_direction:Direction = Direction::Haut;

    match joueur_direction {

        Direction::Haut => println!("On va en haut"),
        Direction::Bas => println!("On va en bas"),
        Direction::Gauche => println!("On va à gauche"),
        Direction::Droite => println!("On va en bas"),

    }

    match toto_direction {

        Direction::Haut => println!("On va en haut"),
        Direction::Bas => println!("On va en bas"),
        Direction::Gauche => println!("On va à gauche"),
        Direction::Droite => println!("On va en bas"),

    }

    

}