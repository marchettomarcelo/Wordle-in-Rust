use colored::*;
use std::io;

fn replace_nth_char(s: &mut String, idx: usize, newchar: char) {
    *s = s
        .chars()
        .enumerate()
        .map(|(i, c)| if i == idx { newchar } else { c })
        .collect()
}

fn update_word_status(guess: &str, secreta: &str, word_status: &mut String) {
    for (guess_letra_index, guess_letra) in guess.trim().chars().enumerate() {
        let secrtea_letra = secreta.chars().nth(guess_letra_index).unwrap();

        if guess_letra == secrtea_letra {
            replace_nth_char(word_status, guess_letra_index, guess_letra);
        } else if secreta.contains(guess_letra) {
            replace_nth_char(word_status, guess_letra_index, '*');
        }
    }
}

fn validate_input(user_guess: &mut String) -> Result<(), &'static str> {
    if user_guess == "opa" {
        return Err("INPUT INVALIDO");
    } else {
        println!("vbom dia")
    }
}

fn game() {
    let mut word_status = String::from("______");
    let secreta = "insper";

    println!(" ");
    println!("{}", "Acete a palavra secreta!:".blue().bold());

    for _ in 0..6 {
        // println!("---------------------------------------------------------------------------");
        println!("{} {}", "Status da palavra:".yellow(), word_status.red());
        println!(
            "{}",
            "---------------------------------------------------------------------------"
        );

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        match guess.len() {
            6 => (),
            _ => {
                println!(
                    "{}",
                    "PALAVRA COM O NUMERO ERRADO DE CARACTERES!!!!!!".red()
                );
                continue;
            }
        };

        update_word_status(&guess, secreta, &mut word_status);

        println!("{}", word_status);

        if guess == secreta {
            // println!("Acertou miseravi");
            break;
        } else {
        }
    }

    if word_status == secreta {
        println!(
            "{}",
            "Voce ganhou o jogo! Parabêns!! :)"
                .green()
                .bold()
                .underline()
        );
        println!("");
        return;
    } else {
        println!("Voce perdeu o jogo :(");
        return;
    }
}
fn main() {
    game()
}
