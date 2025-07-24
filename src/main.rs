// evocando lib padrão input/output
use std::io;
use rand::Rng;

fn main() {
    println!("\n### GUESSING GAME ###\n");
    println!("Entre seu palpite: ");
    
    let mut guess = String::new(); // cria nova string vazia UTF-8 expansível

    // a seguinte lógica pode ser escrita de duas maneiras:
    // io::stdin().read_line(&mut guess).expect("Failed to read line");    
    io::stdin()
        .read_line(&mut guess) // referências também são imutáveis por padrão
        .expect("Failed to read line");

    println!("Palpite: {guess}");
}

