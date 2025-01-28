use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write, ErrorKind};

fn main() {
    let file_name = "fil.txt";

    // Tjek om filen allerede eksisterer (undgå overskrivelse)
    match File::open(file_name) {
        Ok(_) => {
            println!("Filen eksisterer allerede. Ingen ændringer foretaget.");
        }
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                // Opret filen, da den ikke eksisterer
                create_file(file_name);
            } else {
                panic!("Der opstod en fejl: {:?}", error);
            }
        }
    }

    // Menu-loop: Brugeren kan vælge flere handlinger, før programmet afsluttes
    menu_loop(file_name);
}

fn menu_loop(file_name: &str) {
    loop {
        // Udskriv menuen til brugeren
        println!("\nVælg en handling:");
        println!("1. Tilføj tekst til filen");
        println!("2. Læs filens indhold");
        println!("3. Slet filen");
        println!("4. Opret fil på ny (overskriver eksisterende fil)");
        println!("5. Afslut programmet");

        // Læs brugerens valg fra standard input
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Kunne ikke læse input");
        let choice = choice.trim(); // Fjern unødvendige whitespaces

        match choice {
            "1" => append_text(file_name),
            "2" => read_file(file_name),
            "3" => delete_file(file_name),
            "4" => {
                // Opret filen på ny
                create_file(file_name);
            }
            "5" => {
                // Afslut programmet
                println!("Afslutter programmet. Farvel!");
                break;
            }
            _ => {
                // Hvis brugeren indtaster noget ugyldigt
                println!("Ugyldigt valg. Prøv igen.");
            }
        }
    }
}

fn append_text(file_name: &str) {
    let mut file = OpenOptions::new()
        .append(true) // Tilføj tekst til filens slutning (.write er redundant med .append)
        .open(file_name)
        .unwrap_or_else(|_| {
            // Hvis filen ikke findes, opret en ny
            println!("Filen findes ikke. Den bliver oprettet.");
            create_file(file_name)
        });

    println!("Skriv tekst til filen:");
    let mut input = String::new();
    // Læs tekst fra brugeren
    io::stdin()
        .read_line(&mut input)
        .expect("Kunne ikke læse input");

    // Tilføj brugerens input til filen
    file.write_all(format!("\n{}", input).as_bytes())
        .expect("Kunne ikke tilføje tekst til filen");
    println!("Teksten blev tilføjet til filen.");
}

fn read_file(file_name: &str) {
    // Læs og udskriv filens indhold
    match std::fs::read_to_string(file_name) {
        Ok(content) => println!("Filens indhold:\n{}", content),
        Err(_) => println!("Filen kunne ikke læses eller findes ikke."),
    }
}

fn delete_file(file_name: &str) {
    // Slet filen
    match std::fs::remove_file(file_name) {
        Ok(_) => println!("Filen blev slettet."),
        Err(_) => println!("Filen kunne ikke slettes eller findes ikke."),
    }
}

fn create_file(file_name: &str) -> File {
    // Opret fil (eller overskriv eksisterende) og skriv standardtekst
    let mut file = File::create(file_name).expect("Kunne ikke oprette filen");
    file.write_all(b"Dette er en ny fil. ").expect("Kunne ikke skrive til filen");
    println!("Filen blev oprettet med ny tekst.");
    file
}
