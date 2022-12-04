use regex::Regex;

// This works with space separated languages only
pub fn count_words(file: String) {
    let preprocessed_text = preprocess_text(file);
    print!("{}", preprocessed_text);
}

fn preprocess_text(file: String) -> String {
    // For storing results
    let mut _file = file;

    // Removes "" and '' and ,

    // Replaces ... for <s>. This is to make sure that we don't get "lorem ipsum.. <s>"
    _file = apply_regex(&_file, r"(\.\.\.)\s+", String::from(" <s> "));

    // Adds <s> to end/beginning of sentences
    _file = apply_regex(&_file, r"[\.?!]\s+", String::from(" <s> "));

    _file
}

fn apply_regex(file: &String, regexp: &str, replace_with: String) -> String {
    let reg_exp = Regex::new(regexp).unwrap();
    reg_exp.replace_all(&file, replace_with).to_string()
}
