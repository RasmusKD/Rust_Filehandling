use colored::*;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};

fn main() {
    let file_name = "fil.txt";

    // Tjek om filen eksisterer, og opret den hvis nødvendigt
    if File::open(file_name).is_err() {
        println!("Filen findes ikke. Den bliver oprettet.");
        if let Err(e) = create_file(file_name) {
            eprintln!("Fejl ved oprettelse af fil: {}", e);
            return;
        }
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
            println!("3. Tøm filens indhold");
            println!("4. Slet filen");
        } else {
            println!("1. Tilføj tekst til filen {}", "(deaktiveret)".red());
            println!("2. Læs filens indhold {}", "(deaktiveret)".red());
            println!("3. Tøm filens indhold {}", "(deaktiveret)".red());
            println!("4. Slet filen {}", "(deaktiveret)".red());
        }
        println!("5. Opret fil på ny (overskriver eksisterende fil)");
        println!("6. Afslut programmet");

        // Læs brugerens valg fra standard input
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Kunne ikke læse input");
        let choice = choice.trim(); // Fjern unødvendige whitespaces

        match choice {
            "1" if file_exists => {
                if let Err(e) = append_text(file_name) {
                    eprintln!("Fejl: {}", e);
                }
            }
            "2" if file_exists => {
                if let Err(e) = read_file(file_name) {
                    eprintln!("Fejl: {}", e);
                }
            }
            "3" if file_exists => {
                if let Err(e) = clear_file(file_name) {
                    eprintln!("Fejl: {}", e);
                }
            }
            "4" if file_exists => {
                if let Err(e) = delete_file(file_name) {
                    eprintln!("Fejl: {}", e);
                }
            }
            "5" => {
                if let Err(e) = create_file(file_name) {
                    eprintln!("Fejl: {}", e);
                }
            }
            "6" => {
                println!("Afslutter programmet. Farvel!");
                break;
            }
            _ => println!("Ugyldigt valg. Indtast et tal mellem 1-6."),
        }
    }
}

// Tilføj tekst til filen
fn append_text(file_name: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_name)?;

    println!("Skriv tekst til filen:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let input = input.trim(); // Fjern whitespaces og ekstra linjeskift

    if !input.is_empty() {
        writeln!(file, "{}", input)?; // Udskriver tekst med en ny linje
        println!("Teksten blev tilføjet til filen.");
    } else {
        println!("Ingen tekst blev tilføjet.");
    }
    Ok(())
}

// Læs filens indhold
fn read_file(file_name: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_name)?; // Fejlhåndtering gøres automatisk af ?
    if content.is_empty() {
        println!("Filen er tom.");
    } else {
        println!("Filens indhold:\n{}", content);
    }
    Ok(())
}

// Tøm filens indhold uden at slette den
fn clear_file(file_name: &str) -> io::Result<()> {
    File::create(file_name)?; // Overskriver filen uden at skrive noget i den
    println!("Filens indhold er blevet tømt.");
    Ok(())
}

// Slet filen
fn delete_file(file_name: &str) -> io::Result<()> {
    fs::remove_file(file_name)?; // Fejlhåndtering gøres automatisk af ?
    println!("Filen blev slettet.");
    Ok(())
}

// Opret en ny fil (overskriver eksisterende fil)
fn create_file(file_name: &str) -> io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(b"Dette er en ny fil.")?;
    println!("Filen blev oprettet med ny tekst.");
    Ok(())
}
