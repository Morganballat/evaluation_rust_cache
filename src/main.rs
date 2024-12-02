// std est la librairie standard de Rust, car ici nous ne sommes pas autorisé à utiliser les
// librairies externes
// Et io designe le module Input/Output de la librairie standard
use std::io;
// std::io::Read et std::io::Write sont des traits qui définissent les opérations de lecture et d'écriture
use std::io::{Read, Write};
// std::str est le module qui contient les fonctions pour manipuler les chaînes de caractères
use std::str;
// afin de communiquer avec l'API, j'utilise le module standard de rust net
use std::net;

mod structures::city;

// La fonctione main est le point d'entrée de notre programme
fn main() {
    // Demander à l'utilisateur de renseigner le nom d'un pays
    println!("renseignez le code postal d'une ville pour obtenir des informations sur cette dernière");
    // Créer une variable mutable pour stocker le nom du pays
    let mut zip_code = String::new();

    // Lire l'entrée de l'utilisateur et la stocker dans la variable country_name
    // exemple tiré de la documentation rust : https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
    io::stdin().read_line(&mut zip_code).expect("Failed to read line");
   
    // utiliser cet input zip_code pour faire un appel à l'API

    // doc module TcpStream: https://doc.rust-lang.org/std/net/struct.TcpStream.html
    // API : https://www.zippopotam.us/
    let api_url = "api.zippopotam.us";

    //Connexion au serveur de l'api, utilisant le port 80(HTTP), si cela echoue un message d'erreur est affiché
    // doc: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect
    let mut stream = net::TcpStream::connect(format!("{}:80", api_url)).expect("Failed to connect to server");
    // Création de la requête HTTP
    let request = format!("GET /FR/{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", zip_code.trim(), api_url);
    
    // Ecriture de la requête dans le stream. Request est traduit en octets car les flux travaillent
    // avec des données binaires
    // doc: https://doc.rust-lang.org/std/io/trait.Write.html
    stream.write_all(request.as_bytes()).expect("Failed to write to stream");

    // Création d'un vecteur mutable pour stocker la réponse du serveur.
    // les vecteurs sont des tableaux dynamiques en Rust qui permettent de stocker plusieurs valeurs
    // de même type dans une seule variable 
    // doc: https://doc.rust-lang.org/std/vec/struct.Vec.html
    let mut response = Vec::new();
    // Lecture des données dans le flux de la réponse du serveur et stockage dans le vecteur response
    // doc: https://doc.rust-lang.org/std/io/trait.Read.html
    stream.read_to_end(&mut response).expect("Failed to read from stream");

    // Conversion de la réponse en octets en chaîne de caractères UTF-8
    let response_str = str::from_utf8(&response).expect("Failed to convert response to string");
    println!("{}", response_str);

    //Extraction des données de la réponse


    // Fermeture de la connexion
    // doc: https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush
    stream.flush().expect("Failed to flush stream");



}