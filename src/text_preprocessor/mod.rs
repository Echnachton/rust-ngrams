use regex::Regex;

pub fn preprocess_text(file: String) -> String {
    // For storing results
    let mut _file = file;

    // To lower case
    _file = _file.to_lowercase();

    // Removes all non-alphanumeric characters, trims extra spaces, and adds end/beginning tag
    _file = apply_regex(&_file, r"[^A-Za-z0-9\.?!\s]", String::from(""));
    _file = apply_regex(&_file, r"\s+", " ".to_string());
    _file = apply_regex(&_file, r"[\.?!]+\s+", String::from(" <s> "));

    // TODO: Add <s> to beginning and end of file

    _file
}

fn apply_regex(file: &String, regexp: &str, replace_with: String) -> String {
    let reg_exp = Regex::new(regexp).unwrap();
    reg_exp.replace_all(&file, replace_with).to_string()
}
