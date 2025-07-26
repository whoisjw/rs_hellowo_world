use std::fs::*;
use std::io::*;
use std::path::Path;
use rand::prelude::*;

fn main() {
    println!("Time for you to guess the word...");
    let word :String = select_word(&"words.txt");
    println!("{}", word);
    do_wordle(word);
}

fn select_word (file_name: &str) -> String {
    //Create a path to the file
    let path = Path::new(&file_name);
    let display = path.display(); 

    let mut wordlist = match File::open(&path){
        Err(why) => panic!("Error: could not open the file {}: {}", display, why),
        Ok(file) => file,
    };

    let mut word_list_str = String::new();
    match wordlist.read_to_string(&mut word_list_str){
        Err(why) => panic!("Error, could not read {}: {}", display, why),
        Ok(_) => print!("Successfully read {}!", display)
    };

    //now let's read a line from the file!
    let lines :Vec<&str> = word_list_str.lines().collect();
    let chosen_word = lines.choose(&mut rand::rng());

    return chosen_word.expect("big balls and cum").to_string();
}

fn do_wordle(word :String) {
    let mut game_over :bool = false;
    let mut guess = String::new();
    let mut display_word = String::new();

    for i in 0..word.chars().count() {
        display_word.push_str(&"x");
    }

    loop {
        println!("{}\n", display_word);
        println!("Enter your guess: ");
        let b1 :String = std::io::stdin().read_line(&mut guess).unwrap().to_string();

        for i in 1..word.chars().count() {
            if b1.as_bytes()[i] == word.as_bytes()[i] {
                display_word.replace_range(i-1..i, &b1.chars().nth(i).expect("ERROR").to_string());
            }
        }

        if game_over == true {
            break;
        }
    }
}
