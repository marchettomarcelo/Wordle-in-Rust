use std::io;

fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == idx { newchar } else { c })
        .collect()
}

fn game() {
    let mut word_status = String::from("______");
    let secreta = String::from("insper");
    println!("Acete a palavra secreta!:");

    for _ in 0..6 {
        // println!("---------------------------------------------------------------------------");
        println!("Status da palavra: {}", word_status);
        println!("---------------------------------------------------------------------------");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().len() {
            6 => (),
            _ => {
                println!("PALAVRA COM O NUMERO ERRADO DE CARACTERES!!!!!!");
                continue;
            }
        };

        for (letra_index, letra) in guess.trim().chars().enumerate() {
            if letra == secreta.chars().nth(letra_index).unwrap() {
                word_status = replace_nth_char(&word_status, letra_index, letra);
            }
            // println!("{}", secreta.chars().nth(0).unwrap())
        }

        if guess.trim() == secreta {
            println!("Acertou miseravi");
            break;
        } else {
            println!("Nao e a palavra")
        }
    }
    println!("Voce perdeu o jogo :(")
}
fn main() {
    game()
}
