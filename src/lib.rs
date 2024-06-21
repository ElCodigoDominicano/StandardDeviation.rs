/*
 * Required methods and type for all this to work.
 * 
 * Author: AERivas
 * Date: 06/16/2024
 * */

use std::{io::{self, Write}, str::FromStr, num::{ParseIntError, ParseFloatError}, process};

#[derive(Debug)]
pub enum NumberParseError {
    IntError(ParseIntError),
    FloatError(ParseFloatError),
}

impl From<ParseIntError> for NumberParseError {
    fn from(err: ParseIntError) -> Self {
        eprintln!(
            "Error: {:?}, Please try again.",
            err.to_string()
        );
        NumberParseError::IntError(err)
    }
}

impl From<ParseFloatError> for NumberParseError {
    fn from(err: ParseFloatError) -> Self {
        eprintln!(
            "Error: {:?}, Please try again.",
            err.to_string()
        );
        NumberParseError::FloatError(err)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Int(i32),
    Float(f32),
}

impl Number {
    fn get_number(self) -> f32 {
        match self {
            Number::Int(y) => y as f32,
            Number::Float(y) => y, 
        }
    }
}

impl FromStr for Number {
    type Err = NumberParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       if s.contains(".") {
           s.parse::<f32>().map(Number::Float).map_err(NumberParseError::from)
       } else {
           s.parse::<i32>().map(Number::Int).map_err(NumberParseError::from)
       }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserData {
    pub data: Vec<Number>,
}

impl UserData {
   
    fn sum_squared(&self) -> f32 {
        let mut num = 0.0;
        for x in  &self.sqrt() {
            num += x;
        }
        num
    }

    fn sum(&self) -> f32 {
        let mut summed = 0.0;
        for x in &self.data {
            let num = x.clone().get_number();
            summed += num;
        }
        summed
    }

    fn mean(&self) -> f32 {
        self.sum() / self.len()
    }

    fn diff(&self) -> Vec<f32> {
        let mut difference = Vec::new();
        for x in &self.data {
            let mut num = x.clone().get_number();
            num -= self.mean(); 
            difference.push(num);
        }
        difference
    }

    fn sqrt(&self) -> Vec<f32> {
        let mut squared = Vec::new();
        for x in &self.diff() {
            squared.push(x.powf(2.0));
        }
        squared
    }

    fn len(&self) -> f32 { 
        self.data.len() as f32
    }

    fn population_deviation(&self) -> f32 {
        let population = self.sum_squared() / self.len();
        population.powf(0.5)
    }

    fn sample_deviation(&self) -> f32 {
        let sample = self.sum_squared() / (self.len() - 1.0);
        sample.powf(0.5)
    }
}

fn check(user_input_data: &String) {
    if user_input_data.find(" ") == None {
        eprintln!("Apologies.. please rerun the program and enter 2 or more \
            numbers with a space inbetween each number.");
    } else if user_input_data.starts_with(" ") {
        eprintln!("Apologies.. please rerun the program without a space at the start.");
    } else {
        let tes: Vec<Number> = user_input_data
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<Number>()
                .unwrap_or_else(|_| process::exit(1)))
            .collect();

        let user: UserData = UserData { data: tes };
        println!(
            "Population {:?}\nSample {:?}", 
            user.population_deviation(), 
            user.sample_deviation()
        )
    }
}

pub fn user_input() {
    let mut user_input_data = String::new();
    print!("\x1B[2J\x1B[1;1H");
    println!("Standard Deviation Calculator.");
    println!("Please enter a few numbers separated by space.");
    println!("EX: 1 2 3.0 4.5 -5");
    io::stdin()
        .read_line(&mut user_input_data)
        .expect("Failed to read line");
    
    check(&user_input_data) 
}
