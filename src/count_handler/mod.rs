use std::collections::HashMap;

pub fn generate_count_map(file: &String) -> HashMap<String, u32> {
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

pub fn generate_n_gram_count_map(file: &String, n: usize) -> HashMap<String, u32> {
    let split_text: Vec<&str> = file.split(" ").collect();
    let mut word_count_map = HashMap::new();

    let mut index_bottom = 0;
    for index in n..split_text.len() {
        let word = split_text[index_bottom..index].join(" ");
        index_bottom += 1;

        if !word_count_map.contains_key(&word) {
            word_count_map.insert(word.to_string(), 1);
            continue;
        }
        word_count_map
            .entry(word.to_string())
            .and_modify(|count| *count += 1);
    }

    word_count_map
}

pub fn get_possible_word_combination(file: &String, n: usize) -> Vec<String> {
    let split_file: Vec<&str> = file.split(" ").collect();
    let mut index_bottom = 0;
    let mut names = Vec::new();

    for index in n..split_file.len() + 1 {
        let name = split_file[index_bottom..index].join(" ");
        names.push(name);
        index_bottom += 1;
    }

    names
}
