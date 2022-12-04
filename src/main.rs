mod count_handler;
mod io_handler;

use count_handler::count_words;
use io_handler::read_file;

fn main() {
    let file = read_file(String::from("./data/anthem_clean.txt"));
    count_words(file);
}
