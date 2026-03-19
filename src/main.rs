mod models;
use models::{Ticket, TicketType};
use std::io::{self, Write}; 

fn main() {
    let mut liste_tickets: Vec<Ticket> = Vec::new();

    println!("---  BIENVENUE DANS TICKET MAKER ---");

    loop {
        println!("\n--- Création d'un nouveau ticket ---");

        println!("Type (1: Bug, 2: Feature, 3: Doc, 4: Refactor) :");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur");
        let type_ticket = match choix.trim() {
            "2" => TicketType::Feature,
            "3" => TicketType::Documentation,
            "4" => TicketType::Refactor,
            _ => TicketType::Bug,
        };

        let author = prompt("Auteur : @");
        let title = prompt("Titre : ");
        let content = prompt("Description : ");

        let mut tasks = Vec::new();
        println!("Entrez vos tâches (laissez vide pour terminer) :");
        loop {
            let task = prompt("  > Tâche : ");
            if task.is_empty() { break; }
            tasks.push(task);
        }

        let nouveau_ticket = Ticket::new(type_ticket, &author, &title, &content, tasks);
        
        liste_tickets.push(nouveau_ticket);
        println!("✅ Ticket ajouté à la liste.");

        print!("\nVoulez-vous créer un autre ticket ? (Y/n) : ");
        io::stdout().flush().unwrap();
        
        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).expect("Erreur");
        if reponse.trim().to_lowercase() == "n" {
            break; 
        }
    }

    println!("\n--- 💾 SAUVEGARDE DE {} TICKET(S) ---", liste_tickets.len());
    for ticket in &liste_tickets {
        ticket.save().expect("Erreur lors de la sauvegarde");
        println!("Fichier créé : {}.md", ticket.title.to_lowercase().replace(" ", "-"));
    }

    println!("\nTerminé ! Merci d'avoir utilisé Ticket Maker.");
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string() 
}