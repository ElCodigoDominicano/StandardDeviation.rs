/* A small program that calulates the Standard Deviation
 * for a set of numbers.
 *
 * PLEASE NOTE: THIS IS A PERSONAL CHALLENGE PROJECT IT IS NOT COMPLETE.
 * I HAVE NO PRIOR EXPERIENCE OR KNOWLEDGE WITH STATISTICS(unfortunately)....
 *              ____________
    Formula: 𝝈=√ ∑(xi-𝜇)^2
                 ----------
                     N

    In the formula above...
    𝝈 - is the standard deviation
    xi - is each individual data point in a set,
    𝜇 - is the mean.
    ^2 is the result of (xi - 𝜇) squared
    N - is the total number of data points.
    ∑ - is sum of the squared result.

 * Author: AERivas
 * Date: 06/13/2024
 * */

#[derive(Debug, PartialEq, Clone)]
enum Number {
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
struct UserData {
    data: Vec<Number>,
}

impl UserData {
    fn len(&self) -> f32 { 
        self.data.len() as f32
    }

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

    fn population_deviation(&self) -> f32 {
        let population = self.sum_squared() / self.len();
        population.powf(0.5)
    }

    fn sample_deviation(&self) -> f32 {
        let sample = self.sum_squared() / (self.len() - 1.0);
        sample.powf(0.5)
    }
}

fn main() {
    use Number::*;
    // Standard Deviation, σ: 874.32854145338
    // Standard Deviation, s: 921.62320476188
     
    let user_data: UserData = UserData { 
        data: vec![
           Int(1), 
           Float(5.3), 
           Int(200), 
           Float(-533.6), 
           Int(1000), 
           Float(2000.9),
           Int(-1500), 
           Float(-2.5), 
           Int(-50),
           Float(-350.7)
        ]
    };
    
    println!(
        "Population Deviation {:?}\nSample Deviation {:?}",
        user_data.population_deviation(),
        user_data.sample_deviation(),
    );
}
