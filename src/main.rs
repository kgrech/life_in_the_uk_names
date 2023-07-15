use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::Read;
use rand::Rng;
use serde::Deserialize;
use clap::Parser;

#[derive(Debug, Deserialize, Eq, PartialOrd, PartialEq, Hash)]
struct Person {
    name: String,
    years: String,
    profession: String,
    text: String
}

struct Test {
    data: HashMap<String, Vec<Person>>,
    professions: Vec<String>,
}

impl Test {

    pub fn new() -> Self {
        let mut file = File::open("data.json").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let json: Vec<Person> = serde_json::from_str(&data).unwrap();

        let mut data = HashMap::new();
        for person in json {
            if !data.contains_key(&person.profession) {
                data.insert(person.profession.clone(), vec![]);
            }
            data.get_mut(&person.profession).unwrap().push(person);
        }

        let professions = data.keys().map(|s| s.clone()).collect();
        Self {
            data,
            professions,
        }
    }

    pub fn next_question(&self, topic: Option<&str>, reverse: bool) -> Question {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.professions.len());
        let topic = topic.unwrap_or_else(|| &self.professions[idx]);

        let mut rng = rand::thread_rng();

        let persons = self.data.get(topic).unwrap();
        let answers_num = std::cmp::min(4, persons.len());

        let mut texts = HashSet::new();
        let mut answers = Vec::new();
        while answers.len() < answers_num {
            let idx = rng.gen_range(0..persons.len());
            if !texts.contains(persons[idx].text.as_str()) {
                texts.insert(persons[idx].text.as_str());
                answers.push(&persons[idx]);
            }
        }

        let expected_answer = rng.gen_range(0..answers_num);
        Question {
            answers,
            expected_answer,
            reverse
        }
    }
}

pub struct Question<'a> {
    answers: Vec<&'a Person>,
    expected_answer: usize,
    reverse: bool,
}


impl<'a> Question<'a> {

    pub fn print(&self, by_click: bool) {
        println!( "----------------------------------------------");
        if self.reverse {
            println!( "{}", self.answers[self.expected_answer].name);
        } else {
            println!("{}", self.answers[self.expected_answer].text);
        }
        if by_click {
            let mut input_line = String::new();
            io::stdin()
                .read_line(&mut input_line)
                .expect("Failed to read line");
        }
        for i in 0..self.answers.len() {
            if self.reverse {
                println!( "\t{}. {}", i + 1, self.answers[i].text);
            } else {
                println!( "\t{}. {}", i + 1, self.answers[i].name);
            }
        }
    }

    pub fn answer(&self, answer: usize) -> bool {
        let correct = self.expected_answer == answer - 1;
        println!();
        if correct {
            println!("-- CORRECT --");
        } else {
            println!("\x1b[93m-- INCORRECT --\x1b[0m");
        }
        for answer in &self.answers {
            println!("> {}: {}", answer.name, answer.text);
        }
        correct
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    topic: Option<String>,
    #[arg(short, long)]
    reverse: bool,
    #[arg(short, long)]
    by_click: bool
}


fn main() {
    let args: Args = Args::parse();

    let topic = args.topic.as_ref().map(|s| s.as_str());

    let test = Test::new();
    println!("List of topics");
    for (k, v) in &test.data {
        println!("{k}: {}", v.len());
    }

    let mut correct = 0u32;
    let mut total = 0u32;

    loop {
        let question = test.next_question(topic, args.reverse);
        question.print(args.by_click);

        print!("Answer: ");
        let mut input_line = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");

        if input_line == "e\n" {
            return;
        }
        let answer = input_line.trim().parse().expect("Input not is not u32");
        if question.answer(answer) {
            correct += 1;
        }
        total += 1;

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        print!("{}[2J", 27 as char);
        println!("Correct answers {}/{}: {:.2}%", correct, total, correct as f32 / total as f32 * 100.0);
    }
}
