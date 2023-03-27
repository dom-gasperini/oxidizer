/**
 * Passphrase Generator
 * Dominic Gasperini
 * 
 * This program generates random passphrases of varying complexity depending
 * on the level selected by the user. The individual words are pulled from a file that
 * contains about 350,000 legal English words and all numbers and special characters are
 * also randomly chosen.
 */


// includes
use rand::{seq::SliceRandom, Rng};
use std::{io::{self, BufReader, BufRead, Write}, fs::File};
use colored::Colorize;

// constants
const FILENAME: &str = "./word-list.txt";


fn load_words() -> Vec<String>{
    // inits
    let file = File::open(FILENAME).expect("Failed to load in words!");
    let reader = BufReader::new(file);

    // read file
    reader.lines()
        .map(|l| l.expect("couldn't read file"))
        .collect()
}


fn basic(words: &Vec<String>) -> Vec<String> {
    // inits
    let mut passphrases: Vec<String> = Vec::new();
    let mut passphrase = String::from("").to_owned();
    let splitter: &str = "-";
    
    for _ in 0..3 {
        // get 3 random words
        let phrases:Vec<&String> = words.choose_multiple(&mut rand::thread_rng(), 3).collect();

        passphrase.push_str(phrases[0].as_str());
        passphrase.push_str(splitter);
        passphrase.push_str(phrases[1].as_str());
        passphrase.push_str(splitter);
        passphrase.push_str(phrases[2].as_str());

        passphrases.push(passphrase.clone());
        passphrase.clear();
    }

    passphrases
}

fn moderate(words: &Vec<String>) -> Vec<String> {
    // inits
    let mut passphrases: Vec<String> = Vec::new();
    let mut passphrase: String = String::from("").to_owned();
    let splitter: &str = "-";

    for _ in 0..3 {
        // get 3 random words
        let phrases:Vec<&String> = words.choose_multiple(&mut rand::thread_rng(), 3).collect();

        // get random number
        let secret_number: i32 = rand::thread_rng().gen_range(1..=100);        

        passphrase.push_str(phrases[0].as_str());
        passphrase.push_str(secret_number.to_string().as_str());

        passphrase.push_str(splitter);

        let first_char = phrases[1].chars().next().unwrap().to_uppercase();
        let capitalized = format!("{}{}", first_char, &phrases[1][1..]);
        passphrase.push_str(capitalized.as_str());

        passphrase.push_str(splitter);

        passphrase.push_str(phrases[2].as_str());


        // add to list of passphrases and reset
        passphrases.push(passphrase.clone());
        passphrase.clear();
    } 

    passphrases
}

fn advanced(words: &Vec<String>) -> Vec<String> {
    // inits
    let mut passphrases: Vec<String> = Vec::new();
    let mut passphrase: String = String::from("").to_owned();
    let splitter: &str = "-";
    
    for _ in 0..3 {
        // get 3 random words
        let phrases:Vec<&String> = words.choose_multiple(&mut rand::thread_rng(), 4).collect();
        
        // get random number and special character
        let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
        let special_characters = vec!["!", "@", "#", "$", "&", "/"];

        passphrase.push_str(phrases[0].as_str());
        passphrase.push_str(secret_number.to_string().as_str());

        passphrase.push_str(splitter);

        let first_char = phrases[1].chars().next().unwrap().to_uppercase();
        let capitalized = format!("{}{}", first_char, &phrases[1][1..]);
        passphrase.push_str(capitalized.as_str());

        passphrase.push_str(splitter);

        passphrase.push_str(phrases[2].as_str());
        passphrase.push_str(special_characters.choose(&mut rand::thread_rng()).unwrap());


        // add to list of passphrases and reset
        passphrases.push(passphrase.clone());
        passphrase.clear();
    }

    passphrases
}


fn main() {
    // inits
    let mut passphrases:Vec<String> = Vec::new();
    let mut choice:String = String::new();

    // welcome
    println!("\n---passphrase generator---\n");
    println!("select a complexity level: ");
    println!("1. basic      (3 random words)");
    println!("2. moderate   (3 random words + number)");
    println!("3. advanced   (3 random words + number + special character)\n");
    
    print!("enter your choice: ");
    io::stdout().flush().expect("Unable to print line!");

    // get user input
    io::stdin().read_line(&mut choice)
    .ok()
    .expect("Invalid input...");


    // load in words!
    let words = load_words();

    match choice.trim() {
        "1" => passphrases = basic(&words),
        "2" => passphrases = moderate(&words),
        "3" => passphrases = advanced(&words),
        _ => println!("Invalid choice!")
    }
    
    // apply some color and print final passphrase
    println!("\nPassphrases:");
    for i in 0..3 {
        println!("{}", passphrases.get(i).unwrap().green());
    }
    println!();
}
