use std::collections::HashMap;
use colored::Colorize;

pub enum LetterStatus{
    G(String) , Y(String) , R(String) , X(String) ,
}

fn print_letter(letter: &LetterStatus){
    match letter {
        LetterStatus::G(letter) => print!("{}",letter.green()),
        LetterStatus::Y(letter) => print!("{}",letter.yellow()),
        LetterStatus::R(letter) => print!("{}",letter.red()),
        LetterStatus::X(letter) => print!("{}",letter),
    }
}

pub struct Word{
    pub word: String,
    pub word_status: Vec<LetterStatus>,
}

impl Word{
    pub fn new(word: &String)->Self{
        let mut vector = Vec::new();
        let w_arr:Vec<char> = word.chars().collect() ;

        for x in &w_arr{
            vector.push(LetterStatus::X(x.to_string()));
        }
        
        Self { word: word.clone() , word_status: vector}
    }

    pub fn match_answer(self , answer: &String , answer_hash: &mut HashMap<char , i32>)->Self{
        let a_arr:Vec<char> = answer.chars().collect();
        let w_arr:Vec<char> = self.word.chars().collect() ;
        let mut temp_answer_hash = answer_hash.clone();
        let mut vector = self.word_status ;

        
        for i in 0..3{
            for j in 0..self.word.len(){
                match temp_answer_hash.get(&w_arr[j]) {
                    Some(x) if x > &0 => {
                        match i {
                            0 => {
                                if w_arr[j] == a_arr[j] {
                                    vector[j] = LetterStatus::G(w_arr[j].to_string()) ;
                                    let count = temp_answer_hash.entry(w_arr[j]).or_insert(0);
                                    *count -= 1 ;
                                }
                            }
                            1 => {
                                if w_arr[j] != a_arr[j] {
                                    vector[j] = LetterStatus::Y(w_arr[j].to_string()) ;
                                    let count = temp_answer_hash.entry(w_arr[j]).or_insert(0);
                                    *count -= 1 ;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        match &vector[j]{
                            LetterStatus::X(_) => vector[j] = LetterStatus::R(w_arr[j].to_string()) ,
                            _ => {}
                        }
                    }
                }
            }
        }

        Self { word: self.word , word_status: vector }
    }

    pub fn print_colored_word(&self){
        for x in &self.word_status{
            self::print_letter(x);
        }
        print!("\n");
    }
}