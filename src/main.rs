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
    for (guess_letra_index, guess_letra) in guess.chars().enumerate() {
        let secrtea_letra = secreta.chars().nth(guess_letra_index).unwrap();

        if guess_letra == secrtea_letra {
            replace_nth_char(word_status, guess_letra_index, guess_letra);
        } else if secreta.contains(guess_letra) {
            replace_nth_char(word_status, guess_letra_index, '*');
        }
    }
}

fn validate_input(user_guess: &mut String) -> Result<(), &'static str> {
    *user_guess = user_guess.trim().to_owned();
    if user_guess.len() == 6 {
        return Ok(());
    } else {
        return Err("PALAVRA COM O NUMERO ERRADO DE CARACTERES!!!!!!");
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

        match validate_input(&mut guess) {
            Err(err) => {
                println!("{}", err.red());
                continue;
            }
            Ok(_) => (),
        }

        update_word_status(&guess, secreta, &mut word_status);

        println!("{}", word_status);

        if guess == secreta {
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
        }
    }
    println!("Voce perdeu o jogo :( ");
}
fn main() {
    game()
}
