mod count_handler;
mod io_handler;
mod text_preprocessor;

use count_handler::{generate_count_map, generate_n_gram_count_map};
use io_handler::{get_args, read_file};
use text_preprocessor::preprocess_text;

fn main() {
    // Getting args
    let args = get_args();
    let user_input = args[1];

    // Processing and counting the target file
    let file = read_file(String::from("./data/anthem_clean.txt"));
    let processed_file = preprocess_text(file);
    let target_count = generate_count_map(&processed_file);
    let condition_count = generate_n_gram_count_map(&processed_file, 2);

    // Calculating probability

    println!("{:?}", condition_count);
}
