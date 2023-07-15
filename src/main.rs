use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::Read;
use rand::Rng;
use serde::Deserialize;

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

    pub fn next_question(&self) -> Question {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.professions.len());
        let topic = &self.professions[idx];

        let mut rng = rand::thread_rng();

        let persons = self.data.get(topic).unwrap();
        let answers_num = std::cmp::min(4, persons.len());

        let mut answers = HashSet::new();
        while answers.len() < answers_num {
            let idx = rng.gen_range(0..persons.len());
            answers.insert(&persons[idx]);
        }
        let answers = answers.into_iter().collect::<Vec<_>>();
        let expected_answer = rng.gen_range(0..answers_num);
        Question {
            answers,
            expected_answer
        }
    }
}

pub struct Question<'a> {
    answers: Vec<&'a Person>,
    expected_answer: usize
}

impl<'a> Display for Question<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----------------------------------------------")?;
        writeln!(f, "{}", self.answers[self.expected_answer].text)?;
        for i in 0..self.answers.len() {
            writeln!(f, "\t{}. {}", i + 1, self.answers[i].name)?;
        }
        Ok(())
    }
}

impl<'a> Question<'a> {

    pub fn answer(&self, answer: usize) -> bool {
        let correct = self.expected_answer == answer - 1;
        println!();
        if correct {
            println!("-- CORRECT --");
        } else {
            println!("-- INCORRECT --");
        }
        for answer in &self.answers {
            println!("> {}: {}", answer.name, answer.text);
        }
        correct
    }
}


fn main() {
    let test = Test::new();

    let mut correct = 0u32;
    let mut total = 0u32;

    loop {
        let question = test.next_question();
        println!("{question}");

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
        println!("Correct answers {}/{}: {:.2}%", correct, total, correct as f32 / total as f32 * 100.0);
        println!();
        println!();
    }
}
