mod count_handler;
mod io_handler;
mod text_preprocessor;

use count_handler::{generate_count_map, generate_n_gram_count_map, get_possible_word_combination};
use io_handler::{get_args, read_file};
use text_preprocessor::preprocess_text;

fn main() {
    // Getting args
    let args = get_args();
    let user_input = &args[1];
    let n = args[2].parse::<usize>().unwrap();

    // Processing and counting the target file
    let file = read_file(String::from("./data/anthem_clean.txt"));
    let processed_file = preprocess_text(file);
    let target_count_map = generate_count_map(&processed_file);
    let condition_count_map = generate_n_gram_count_map(&processed_file, n);

    // Calculating probability
    let names = get_possible_word_combination(user_input, n);
    let mut accumulated_prob = 1.0;

    for n_gram in names {
        let target = n_gram.split(" ").collect::<Vec<&str>>()[0];
        let target_count = match target_count_map.get(target) {
            Some(i) => *i as f32,
            // TODO: Add smoothing
            None => 1.0,
        };
        let condition_count = match condition_count_map.get(&n_gram) {
            Some(i) => *i as f32,
            // TODO: Add smoothing
            None => 1.0,
        };
        let probability = condition_count / target_count;
        let log_prob = probability.log10().abs();
        accumulated_prob = accumulated_prob + log_prob;

        // This logs the probability of each n-gram
        println!("{}: {:?}", n_gram, probability)
    }

    // This logs the total probability of the word sequence
    println!("Total Log Probability: {:?}", accumulated_prob);
}
