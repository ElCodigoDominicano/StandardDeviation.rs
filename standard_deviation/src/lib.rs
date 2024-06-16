/*
 * Required methods and type for all this to work.
 * 
 * Author: AERivas
 * Date: 06/16/2024
 * */

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

    pub fn population_deviation(&self) -> f32 {
        let population = self.sum_squared() / self.len();
        population.powf(0.5)
    }

    pub fn sample_deviation(&self) -> f32 {
        let sample = self.sum_squared() / (self.len() - 1.0);
        sample.powf(0.5)
    }
}


