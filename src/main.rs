use std::fs::File;
use std::io::{BufRead,BufReader};
use std::io;
use rand::Rng;
use std::{collections::HashSet};

static QUESTION_FILE: &str = "quiz_questions.csv";

fn n_lines() -> i32 {
    let mut output:usize = 0;
    let file = match File::open(QUESTION_FILE){
        Err(E) => panic!("could not open file {} : {}", QUESTION_FILE, E),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    output = reader.lines().count();
    output as i32
}

fn get_line(n:i32, max_n:i32) -> String {
    let mut output: String = "".to_string();    
    if n < max_n {
        let mut count: i32 = 0;
        let mut n_lines: i32 = 0;
        let file = match File::open(QUESTION_FILE){
            Err(E) => panic!("could not open file {} : {}", QUESTION_FILE, E),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);

        for line in reader.lines(){
            if n == count {
                output = line.unwrap();
            }
            count+=1;
            n_lines+=1;
        }
    }
    output
}

fn read_questions() -> Result<(), std::io::Error> {
    let file = match File::open(QUESTION_FILE){
        Err(E) => panic!("could not open file {} : {}", QUESTION_FILE, E),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let output = line?;
        let parts = output.split(",");
        for p in parts {
            println!("{}",p);
        }
    }
    Ok(())    
}

fn rand_questions(n:i32,max_n:i32) -> Vec<i32> {
    let mut i_count = 0;
    let mut unique_values: HashSet<i32> = HashSet::new();
    while i_count < n {
        let r = rand::thread_rng().gen_range(0..max_n);
        if unique_values.insert(r) { // returns true if successful
            i_count+=1
        }
    }
    Vec::from_iter(unique_values) 
}

fn guess_game(question: String, answer: String) -> bool{
    println!("{}",question);
    println!("your guess:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    //println!("You guessed: {guess}")
    let guess: String = match guess.trim().parse() {
        Ok(str) => str,
        _ => "".to_string() // correct answer cannot be zero, but continue 
    };
        
    match guess {
        guess if guess == answer => {
            println!("you guessed {guess} - that is correct!");
            true
        }
        guess if guess != answer => {
            println!("you guessed {guess} - that is incorrect! the answer is {answer}");
            false
        }
        _ => {
            println!("something went wrong!");
            false
        }        
    }
}

fn set_number_qs(max: i32) -> i32 {
    println!("Welcome to quiz-game how many questions would you like?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        _ => {
            println!("you need to enter a number!");
            0
        } // correct answer cannot be zero, but continue 
    };

    if guess < max {
        guess
    }
    else 
    {
        0
    }
        
}

fn main() {

    let max_n = n_lines();

    let mut n_questions = 0;
    while n_questions == 0 {
        n_questions = set_number_qs(max_n);
    }
    let q_index = rand_questions(n_questions, max_n);

    let mut points = 0;
    for i in q_index {
        //println!("{}", get_line(i,test1_max));
        let line = get_line(i,max_n);
        let next_q: Vec<_> = line.split(",").collect::<Vec<_>>();
        let q = next_q[0].to_string();        
        let a = next_q[1].to_string();
        if guess_game(q, a){points+=1}        
    }
    println!("quiz finished, total score = {}/{}",points,n_questions);
}
