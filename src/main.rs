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
    for (letra_index, letra) in guess.trim().chars().enumerate() {
        if letra == secreta.chars().nth(letra_index).unwrap() {
            replace_nth_char(word_status, letra_index, letra);
        } else if secreta.contains(letra) {
            replace_nth_char(word_status, letra_index, '*');
        } else {
            replace_nth_char(word_status, letra_index, '_')
        }
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
