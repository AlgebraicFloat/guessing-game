// Create a function that finds the average # of turns using the optimal strategy
// Create a function that finds the maximum # of turns using the optimal strategy.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
   let mut points = String::new();
   println!("How many points do you want to plot? Ex: 100");
   io::stdin().read_line(&mut points).expect("Failed to read line!");
   let points: u32 = points.trim().parse().expect("Not an integer");

   let mut total_trials = String::new();
   println!("How many trials do you want to do per point? Ex: 1000000");
   io::stdin().read_line(&mut total_trials).expect("Failed to read line!");
   let total_trials: f64 = total_trials.trim().parse().expect("Not an integer");

   println!("Average # of turns:");
   for x in 1..=points{
       average_turns(x, total_trials);
   }

   println!("\nMax number of turns"); // Total trials is set to 1000.
   for x in 1..=points{
       max_turns(x)
   }
}

fn average_turns(x: u32, total_trials: f64) {
   let mut wins: f64 = 0.0;
   let mut total_turns: f64 = 0.0;
   while wins < total_trials {
       let secret_number = rand::thread_rng().gen_range(1..=x);
       let mut a = 0;
       let mut b = x;
       loop {
           total_turns += 1.0;
           let guess = (a+b)/2; // always rounds down.
           match guess.cmp(&secret_number) {
               Ordering::Less => {a = guess + 1} // println!("\n{} is too small!", guess)
               Ordering::Greater => {b = guess} // println!("\n{} is too big!", guess)
               Ordering::Equal => {wins += 1.0; break} // println!("\n{} is correct!", guess)
           }
       }
   }
   let average: f64 = total_turns/total_trials;
   println!("{:?}", average);
}

fn max_turns(x: u32) {
   let mut wins: u64 = 0;
   let mut max_turns = 0;
   while wins < 1000 {
       let secret_number = rand::thread_rng().gen_range(1..=x);
       let mut a = 0;
       let mut b = x;
       let mut current_turns: u64 = 0;
       loop {
           current_turns += 1;
           let guess = (a+b)/2; // always rounds down.
           match guess.cmp(&secret_number) {
               Ordering::Less => {a = guess + 1} // println!("\n{} is too small!", guess)
               Ordering::Greater => {b = guess} // println!("\n{} is too big!", guess)
               Ordering::Equal => {wins += 1; break} // println!("\n{} is correct!", guess)
           }
       }
       if current_turns > max_turns {max_turns = current_turns}
   }
   println!("{}", max_turns)
}
