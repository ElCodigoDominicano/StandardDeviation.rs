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

fn main() {
    // Standard Deviation, σ: 874.32854145338
    // Standard Deviation, s: 921.62320476188
    let test: Vec<Number> = vec![
        Number::Int(1), 
        Number::Float(5.3), 
        Number::Int(200), 
        Number::Float(-533.6), 
        Number::Int(1000), 
        Number::Float(2000.9),
        Number::Int(-1500), 
        Number::Float(-2.5), 
        Number::Int(-50),
        Number::Float(-350.7)
    ];
    standard_deviation(test);
}

fn standard_deviation(data: Vec<Number>) {
    let sum_of_data = sum(data.clone());
    let len_of_data = data.len();
    let mean_of_data = mean(sum_of_data, len_of_data);
    let difference = diff(data, mean_of_data);
    let squared = sqrt(difference);
    let squared_sum = sum_squared(squared);
    
    let population_standard_deviation = squared_sum / len_of_data as f32;
    let sample_standard_deviation = squared_sum / (len_of_data as f32 - 1.0);
    
    println!("This is the population standard deviation {:?}", population_standard_deviation.powf(0.5));
    println!("This is the sample standard deviation {:?}", sample_standard_deviation.powf(0.5));
}

fn sum_squared(arr: Vec<f32>) -> f32 {
    let mut num = 0.0;
    for x in arr {
        num += x;
    }
    num
}

fn sum(arr: Vec<Number>) -> f32 {
    let mut summed = 0.0;
    for x in arr {
        let num = x.get_number();
        summed += num;
    }
    summed
}

fn mean(sum_of_data: f32, len_of_data: usize) -> f32 {
    sum_of_data / len_of_data as f32
}

fn diff(arr: Vec<Number>, mean_data: f32) -> Vec<f32> {
    let mut difference = Vec::new();
    for x in arr {
        let mut num = x.get_number();
        num -= mean_data; 
        difference.push(num);
    }
    difference
}

fn sqrt(arr: Vec<f32>) -> Vec<f32> {
    let mut squared = Vec::new();
    for x in arr {
       squared.push(x.powf(2.0));
    }
    squared
}
