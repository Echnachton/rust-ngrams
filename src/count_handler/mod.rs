use std::collections::HashMap;

// This works with space separated languages only
pub fn count_words(file: String) -> HashMap<String, u32> {
    let word_count = generate_count_struct(&file);
    word_count
}

fn generate_count_struct(file: &String) -> HashMap<String, u32> {
    let split_text = file.split(" ");
    let mut word_count_map = HashMap::new();

    for word in split_text {
        if !word_count_map.contains_key(word) {
            word_count_map.insert(word.to_string(), 1);
            continue;
        }
        word_count_map
            .entry(word.to_string())
            .and_modify(|count| *count += 1);
    }

    word_count_map
}
