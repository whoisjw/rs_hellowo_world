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
    let mut count = 0;
    let mut b1;
    let mut b2;

    for i in 0..word.chars().count() {
        display_word.push_str(&"x");
    }

    loop {
        println!("{} - {}/10 guesses taken", display_word, count.to_string());
        println!("Enter your guess: ");

        guess.clear();
        display_word.clear();
        std::io::stdin().read_line(&mut guess).unwrap();

        println!("{} is your guess", &guess);

        for x in 0..word.chars().count() {
            if guess.as_bytes()[x] == word.as_bytes()[x] {
                b1 = guess.as_bytes()[x];
                b2 = b1 as char;
                display_word.push_str(&b2.to_string());
            }
            else {
                display_word.push_str(&"x");
            }
        }

        count += 1;

        if guess.trim() == word.trim() {
            println!("Woah, you won!");
            game_over = true;
        }
        else if count == 10 {
            println!("You have ran out of guesses, now I am going to find and kill you irl!");
        }

        if game_over == true {
            break;
        }
    }
}
