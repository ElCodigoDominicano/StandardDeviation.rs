/* A small program that calulates the Standard Deviation
 * for a set of numbers.
 *
 * PLEASE NOTE: THIS IS A PERSONAL CHALLENGE PROJECT IT IS NOT COMPLETE.
 * I HAVE NO PRIOR EXPERIENCE OR KNOWLEDGE WITH STATISTICS(unfortunately)....
 *
    
    watching a @Primeagen video, the sir stated "go learn the standard
    deviation.." it was a video about software engineering *thinking* cant remember
    at that point, fixated on the word "Standard Deviation" stopped watching 
    @Primeagen and got inspired to just "Not search for a function." but "To search 
    for its formula and build a function based on instructions." 

    NOTE: easier said than done..

    instructions obtained watching 2 very informative youtubers on YouTube
    @SirBernieReyes and @TheOrganicChemistryTutor their breakdown, explaination 
    on Variance and Standard Deviation and solving a problem using the formula.
    
                           ____________
    Formula Population: 𝝈=√ ∑(xi-𝜇)^2
                            ----------
                                N
                            ____________
    Formula Population: s=√ ∑(xi-overlinex)^2
                            ----------
                              N - 1
                
                
    In the formula above...
    𝝈 - is the standard deviation
    xi - is each individual data point in a set,
    𝜇 - is the mean.
    ^2 is the result of (xi - 𝜇) squared
    N - is the total number of data points.
    ∑ - is sum of the squared result.
    
 * Author: AERivas
 * Date: 06/13/2024 Updated: 06/16/2024
 * */

use standard_deviation::{Number, UserData};

fn main() {

    // Standard Deviation, σ: 874.32854145338
    // Standard Deviation, s: 921.62320476188
    let dataset: Vec<Vec<Number>> = vec![
        [Number::Int(-3), Number::Int(3), Number::Int(-5), Number::Int(9)].to_vec(),
        [
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
        ].to_vec(),
        [
            Number::Float(5.3), 
            Number::Float(-533.6), 
            Number::Float(2000.9),
            Number::Float(-2.5), 
            Number::Float(-350.7)
        ].to_vec(),
        [
            Number::Int(76), 
            Number::Int(84),
            Number::Int(69),
            Number::Int(92),
            Number::Int(58),
            Number::Int(89),
            Number::Int(73),
            Number::Int(97), 
            Number::Int(85), 
            Number::Int(77),
        ].to_vec(),
    ]; 
    
    for x in dataset {
        let user_data: UserData = UserData {
            data: x,
        }; 
        println!(
            "Population Deviation {:?}\nSample Deviation {:?}",
            user_data.population_deviation(),
            user_data.sample_deviation(),
        );
    };
}
