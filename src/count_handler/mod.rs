use regex::Regex;
use std::collections::HashMap;

// This works with space separated languages only
pub fn count_words(file: String) {
    let preprocessed_text = preprocess_text(file);
    let word_count = generate_count_struct(&preprocessed_text);
    print!("{:?}", word_count);
}

// TODO: Refactor this
fn preprocess_text(file: String) -> String {
    // For storing results
    let mut _file = file;

    // Removes "" and '' and ,
    _file = apply_regex(&_file, r"'", String::from(""));
    _file = apply_regex(&_file, r"[“”]", String::from(""));
    _file = apply_regex(&_file, r#"""#, String::from(""));

    // Replaces ... for <s>. This is to make sure that we don't get "lorem ipsum.. <s>"
    _file = apply_regex(&_file, r"(\.\.\.)\s+", String::from(" <s> "));

    // Adds <s> to end/beginning of sentences
    _file = apply_regex(&_file, r"[\.?!]\s+", String::from(" <s> "));

    // Removes ,:; so that "city," and "city" aren't counted separately
    _file = apply_regex(&_file, r"[,:;]", "".to_string());

    // Changes all spaces to single spaces
    _file = apply_regex(&_file, r"\s+", " ".to_string());

    // To lower case
    _file = _file.to_lowercase();

    _file
}

fn apply_regex(file: &String, regexp: &str, replace_with: String) -> String {
    let reg_exp = Regex::new(regexp).unwrap();
    reg_exp.replace_all(&file, replace_with).to_string()
}

fn generate_count_struct(file: &String) -> HashMap<&str, u32> {
    let split_text = file.split(" ");
    let mut word_count_map = HashMap::new();

    for word in split_text {
        if !word_count_map.contains_key(word) {
            word_count_map.insert(word, 1);
            continue;
        }
        word_count_map.entry(word).and_modify(|count| *count += 1);
    }

    word_count_map
}

fn sort_map() {}
