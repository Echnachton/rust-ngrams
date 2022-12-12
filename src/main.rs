mod io_handler;
mod n_gram_handler;
mod text_preprocessor;

use io_handler::{get_args, read_file};
use n_gram_handler::calculate_probability;
use text_preprocessor::preprocess_text;

fn main() {
    let file = read_file(String::from("./data/anthem_clean.txt"));
    let processed_file = preprocess_text(file);
    let args = get_args();
    let prob = calculate_probability(args[1].to_string(), processed_file, 2);
    print!("{}", prob);
}
