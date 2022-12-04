use std::fs;

pub fn read_file(path: String) -> String {
    // Reads the file
    let file_result = fs::read_to_string(&path);

    // Returns the contents of the file or throws an error
    let file_contents = match file_result {
        Ok(contents) => contents,
        Err(e) => panic!("There was a problem reading file at {}: {}", path, e),
    };

    // Returns the contents of the file if there was no error
    file_contents
}
