use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Set {
    name: String,
    card_list: HashMap<String, String>,
}

impl Set {
    fn new(name: String) -> Self {
        Self {
            name,
            card_list: HashMap::<String, String>::with_capacity(10)
        }
    }

    fn add_card(&mut self, question: &str, answer: &str) {
        match self.card_list.insert(question.to_string(), answer.to_string()) {
            Some(_) => println!("Card with question '{}' already exists ...", question),
            None => println!("Question '{}' added successfully ...", question), 
        }
    }

    fn remove_card(&mut self, question: &str) {
        match self.card_list.remove(question) {
            Some(_) => println!("Removed question '{}' from {}.", question, self.name),
            None => println!("Card with question '{}' not present in {}.", question, self.name)
        }
    }

    fn display_questions_and_answers(&self) {
        for (question, answer) in self.card_list.iter() {
            println!("Question: {}, Answer: {}", question, answer)
        }
    }

    fn quiz(&self) -> f32 {
        let mut correct_answers = 0;

        for (question, answer) in self.card_list.iter() {
            println!("{}", question);
            let answer_input = get_stdin().unwrap();
            if &answer_input == answer {
                println!("Correct!");
                correct_answers += 1;
            }
            else {
                println!("Incorrect!")
            }
        }

        ((correct_answers / self.card_list.len()) * 100) as f32
    }
}

fn get_stdin() -> Result<String, io::Error> {
    let mut input = String::new();

    loop {
       match io::stdin().read_line(&mut input) {
        Ok(_) => { break }
        Err(_) => eprintln!("There was an error, please try again...")
       }
    }
    input.pop(); // remove trailing new line

    Ok(input) // finally return a valid String
}

fn main() {
    let mut card_set: Set = Set::new(String::from("My Set"));

    let question: String = get_stdin().unwrap();
    let answer: String = get_stdin().unwrap();

    card_set.add_card(&question, &answer);
    
    println!("{}%", card_set.quiz().to_string());
}
