use regex::Regex;

pub fn calculate_probability(target: String, file: String, mut n: usize) -> f32 {
    let target_split: Vec<&str> = target.split(" ").collect();
    let target_split_len = target_split.len();
    let mut accumulated_probability: f32 = 1.0;

    if n <= 1 {
        n = 2;
    }
    let mut index = n - 1;

    while target_split_len > index {
        accumulated_probability = accumulated_probability
            * (count_matches(
                target_split[(index + 1 - n as usize)..(index as usize)].join(" "),
                &file,
            ) / count_matches(target_split[index].to_string(), &file));
        index += 1;
    }

    accumulated_probability
}

fn count_matches(target: String, file: &String) -> f32 {
    let reg_exp = Regex::new(&format!(r"({})", target)).unwrap();
    reg_exp.find_iter(file).count() as f32
}
