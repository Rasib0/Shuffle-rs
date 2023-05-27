use rand::seq::SliceRandom;
use rand::thread_rng;
 use std::env;

fn main() {
         // Get the input string from command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
    println!("Usage: cargo run -- <input_string>");
        return;
    }
    let input_string = &args[1];

    //let input_string = "1234567890qzdynfoem,./[]{}/|!@#$%^&*()";
    
    print!("{}", shuffle_string(input_string));
    println!("{}", shuffle_string(input_string));

}

fn shuffle_string(input_string: &str) -> String {

    // Convert the input string to a character vector
    let mut chars: Vec<char> = input_string.chars().collect();
    let mut rng = thread_rng();
    // Shuffle the characters using the Fisher-Yates algorithm
    chars.shuffle(&mut rng);
    // Convert the shuffled characters bak to a string
    let shuffled_string: String = chars.into_iter().collect();

    shuffled_string
}
