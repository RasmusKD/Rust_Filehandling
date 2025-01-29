use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read};

fn main() {
    let file_name = "fil.txt";

    // Tjek om filen eksisterer, og opret den hvis nødvendigt
    if File::open(file_name).is_err() {
        println!("Filen findes ikke. Den bliver oprettet.");
        create_file(file_name);
    }

    // Menu-loop: Brugeren kan vælge flere handlinger, før programmet afsluttes
    menu_loop(file_name);
}

fn menu_loop(file_name: &str) {
    loop {
        let file_exists = fs::metadata(file_name).is_ok();
        // Udskriv menuen til brugeren
        println!("\nVælg en handling:");
        if file_exists {
            println!("1. Tilføj tekst til filen");
            println!("2. Læs filens indhold");
        } else {
            println!("1. Tilføj tekst til filen (deaktiveret)");
            println!("2. Læs filens indhold (deaktiveret)");
        }
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
            "1" if file_exists => append_text(file_name),
            "2" if file_exists => read_file(file_name),
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
        .expect("Kunne ikke åbne filen");

    println!("Skriv tekst til filen:");
    let mut input = String::new();
    // Læs tekst fra brugeren
    io::stdin()
        .read_line(&mut input)
        .expect("Kunne ikke læse input");

    // Tilføj brugerens input til filen \n for ny linje
    file.write_all(format!("\n{}", input).as_bytes())
        .expect("Kunne ikke tilføje tekst til filen");
    println!("Teksten blev tilføjet til filen.");
}

fn read_file(file_name: &str) {
    // Læs og udskriv filens indhold
    match fs::read_to_string(file_name) {
        Ok(content) => println!("Filens indhold:\n{}", content),
        Err(_) => println!("Filen kunne ikke læses eller findes ikke."),
    }
}

fn delete_file(file_name: &str) {
    // Slet filen
    match fs::remove_file(file_name) {
        Ok(_) => println!("Filen blev slettet."),
        Err(_) => println!("Filen kunne ikke slettes eller findes ikke."),
    }
}

fn create_file(file_name: &str) {
    let mut file = File::create(file_name).expect("Kunne ikke oprette filen");
    file.write_all(b"Dette er en ny fil. ").expect("Kunne ikke skrive til filen");
    println!("Filen blev oprettet med ny tekst.");
}
