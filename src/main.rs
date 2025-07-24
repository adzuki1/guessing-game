// evocando lib padrão input/output
use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("\n### GUESSING GAME ###\n");

    let num = rand::thread_rng()
        .gen_range(1..=100);

    println!("Entre seu palpite: ");

    let mut guess = String::new(); // cria nova string vazia UTF-8 expansível

    // a seguinte lógica pode ser escrita de duas maneiras:
    // io::stdin().read_line(&mut guess).expect("Failed to read line");    
    io::stdin()
        .read_line(&mut guess) // referências também são imutáveis por padrão
        .expect("Failed to read line");

    println!("Palpite: {guess}");

    match guess.cmp(&num){
        Ordering::Less => println!("Muito pequeno!"),
        Ordering::Greater => println!("Muito grande!"),
        Ordering::Equal => println!("na medida certa!"),
    }
}

