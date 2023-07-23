use rand::Rng;
use std::collections::HashMap;
use colored::Colorize;

use super::status::Word;
use super::super::words;
use super::status::LetterStatus;

pub struct Game{
    pub answer:String,
    pub answer_hash: HashMap<char , i32>,

    guess:Vec<Word>,
    alphabet: Word,

    // dictionary:Vec<String>,
    dictionary_hash: HashMap<String , bool>,
    // target:Vec<String>,
    // target_hash: HashMap<String , bool>,

    over: bool,
}

impl Game{
    pub fn new()->Self{
        let answer = words::TAR[rand::thread_rng().gen_range(0..words::TAR.len())].to_string() ;
        let mut answer_hash = HashMap::new() ;

        let mut dictionary_vec  = Vec::new() ;
        let mut dictionary_hash = HashMap::new() ;
        let mut target_vec  = Vec::new() ;
        let mut target_hash = HashMap::new() ;
        let alphabet = Word::new(&"abcdefghijklmnopqrstuvwxyz".to_string());

        for x in words::DIC {dictionary_vec.push(x.to_string()); dictionary_hash.entry(x.to_string()).or_insert(true);}
        for x in words::TAR {target_vec.push(x.to_string()); target_hash.entry(x.to_string()).or_insert(true);}
        for x in answer.chars() {
            print!("{x}: ");
            let count = answer_hash.entry(x).or_insert(0) ;
            *count += 1 ;
            println!("{}",count);
        }
        Self { 
            answer: answer, guess: Vec::new(), 
            // dictionary: dictionary_vec, target: target_vec ,
            answer_hash: answer_hash , dictionary_hash: dictionary_hash ,
            // target_hash: target_hash,
            alphabet: alphabet , over: false ,
        }
    }

    pub fn input_guess(&mut self){
        let mut input = String::new() ;

        loop {
            print!("\n") ;
            println!("{}" , "Enter your word guess(5 letters) and press ENTER".blue()) ;
            std::io::stdin().read_line(& mut input).expect("read line failed");
            input = input.trim().to_string().to_lowercase();

            if input.len() == 5 && self.dictionary_hash.get(&input).is_some(){
                let input_word = Word::new(&input).match_answer(&self.answer, &mut self.answer_hash);
                self.update_alphabet(&input_word);
                self.over = self.is_corrct(&input_word) ;
                self.guess.push(input_word) ;

                if self.guess.len() >= 6 {self.over = true; }

                break () ;
            }

            print!("{}:",input.red());
            if input.len() != 5 {print!("input is not 5 length\n") ;}
            else if self.dictionary_hash.get(&input).is_none() {print!("input is not a word\n") ;}
            input.clear();
        }
    }

    pub fn update_alphabet(&mut self , input_word: &Word){
        for i in 0..input_word.word_status.len(){
            for j in 0..self.alphabet.word_status.len(){
                match (&input_word.word_status[i] , &mut self.alphabet.word_status[j]){
                    (LetterStatus::G(x) , LetterStatus::R(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::G(x.clone()) ;
                    }
                    (LetterStatus::G(x) , LetterStatus::Y(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::G(x.clone()) ;
                    }
                    (LetterStatus::G(x) , LetterStatus::X(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::G(x.clone()) ;
                    }
                    
                    (LetterStatus::Y(x) , LetterStatus::Y(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::Y(x.clone()) ;
                    }
                    (LetterStatus::Y(x) , LetterStatus::X(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::Y(x.clone()) ;
                    }
                    
                    (LetterStatus::R(x) , LetterStatus::X(y) ) if x == y => {
                        self.alphabet.word_status[j] = LetterStatus::R(x.clone()) ;}
                    _ => {}
                }
            }
        }
    }

    pub fn is_corrct(&self , input_word: &Word)->bool{
        let mut flag = true ;

        for i in 0..input_word.word_status.len(){
            match &input_word.word_status[i] {
                LetterStatus::G(_) => {}
                _ => {flag = false ;}
            }
        }

        return flag ;
    }

    pub fn display_guess(&self){
        print!("\n") ;
        for i in 0..self.guess.len(){
            print!("{}. " , i + 1) ;
            self.guess[i].print_colored_word();
        }
        self.alphabet.print_colored_word();
    }

    pub fn is_game_over(&self)->bool{
        return self.over ; 
    }

    pub fn game_run(&mut self){
        loop {
            self.input_guess() ;
            self.display_guess() ;
            if self.is_game_over(){
                println!("corrct answer is {}",self.answer);
                break ();
            }
        }
    }
}