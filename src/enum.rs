fn main() {
    
    enum Direction {
    Haut,
    Bas,
    Gauche,
    Droite
    }

    let player_direction:Direction = Direction::Bas;

    match player_direction {
    Direction::Haut => println!("On va en haut !"),
    Direction::Bas => println!("On va en bas !"),
    Direction::Gauche => println!("On va à Gauche !"),
    Direction::Droite => println!("On va à Droite !"),
    }


}