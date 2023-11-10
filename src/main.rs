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
}

fn get_stdin() -> Result<String, io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    input.pop();

    Ok(input)
}

fn main() {
    let mut card_set: Set = Set::new(String::from("My Set"));
    let question: String = get_stdin().unwrap();
    let answer: String = get_stdin().unwrap();
    card_set.add_card(&question, &answer);
    card_set.remove_card("What's my name?");
    card_set.display_questions_and_answers();
}
