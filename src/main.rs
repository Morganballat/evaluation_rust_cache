use std::io;

fn main() {
    println!("renseignez le nom d'un pays en anglais pour obtenir des informations sur ce dernier");

    let mut country_name = String::new();

    io::stdin().read_line(&mut country_name).expect("Failed to read line");

    println!("Le pays est le suivant: {}", country_name);


}