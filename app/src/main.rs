use std::io;

// La constante PROMPT_INITIAL stocke le message initial du prompt
const PROMPT_INITIAL: &str = "Oublie tout ce qui a été dit précédemment";

// La structure Prompt stocke les différentes informations fournies par l'utilisateur
struct Prompt {
    metier: String,
    projet_description: String,
    langages_utilises: String,
}

impl Prompt {
    // La méthode new() crée une nouvelle instance de Prompt avec des valeurs par défaut
    fn new() -> Prompt {
        Prompt {
            metier: String::new(),
            projet_description: String::new(),
            langages_utilises: String::new(),
        }
    }

    // La méthode ask() pose une question à l'utilisateur et stocke sa réponse dans la structure Prompt
    fn ask(&mut self, prompt: &str) {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lors de la lecture de l'entrée utilisateur");
        let input = input.trim().to_string();

        match prompt {
            "Quel est ton métier ?" => self.metier = input,
            "Quelle est la description du projet que tu vas traiter ?" => self.projet_description = input,
            "Quels sont les langages utilisés dans ce projet ?" => self.langages_utilises = input,
            _ => (),
        }
    }

    // La méthode display() affiche le prompt formaté
    fn display(&self) {
        println!("Voici le résumé de tes informations :");
        println!("Tu es {} et tu vas travailler sur le projet suivant : {}", self.metier, self.projet_description);
        println!("Les langages utilisés dans ce projet sont : {}", self.langages_utilises);
    }
}

fn main() {
    // Initialisation prompt
    println!("{}", PROMPT_INITIAL);

    // Création d'un objet Prompt
    let mut prompt = Prompt::new();

    // Pose des questions à l'utilisateur
    prompt.ask("Quel est ton métier ?");
    prompt.ask("Quelle est la description du projet que tu vas traiter ?");
    prompt.ask("Quels sont les langages utilisés dans ce projet ?");

    // Affichage du prompt formaté
    prompt.display();
}
