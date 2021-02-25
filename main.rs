use std::io; // appelle a la biblio 
use rand::Rng; // appelle à la dépande rand 
use std::cmp::Ordering;  // pour ma comparaison 

fn main() {
    println!("----------------------JEUX DU PLUS OU MOINS------------------------");
//---------------------- Debut de mon programe ----------------------------------
  
    // un choix alleatoir sera fais par la machine 
    let nombre_secret = rand::thread_rng().gen_range(0, 100); 

    loop { // fonctionne un peux comme tant que

//saisie au clavier 
let mut supposition = String::new();
   
    println!("Devinez un nombre secret ! : Saisir le nombre que vous avez deviné ");
        io::stdin()
            .read_line(&mut supposition)
            .expect("Echec de la lecture de l'entré de l'UTILISATEUR ");

            // Le masquage (shadowing). ont vas swhitcher de type String et Int sans cree une autre variable supposition*
            let supposition: u32 = match supposition.trim().parse() {
                Ok(nombre) => nombre,
                Err(_) => continue,
            };

            //on vas afficher la suppositions
   println!("Votre Nombre est : {}", supposition);
//-------------- cette partie du code vas comparer la supposition et nombre_secret----------------------
    match supposition.cmp( &nombre_secret ) {
        Ordering::Less => println!("C'est plus que cela :(" ),
        Ordering::Greater => println!("C'est moins que cela  "),
        Ordering::Equal => {
            println!("Bravo vous avez deviné le Bon Nombre :)");
            break;
        }
        
    }
    
}
// nous affichons le nombre a la fin du programme 
println!("Le nombre Attendu etait : {}", nombre_secret );

}
