use std::fs;

pub fn get_clean_input(day_number: &str) -> Result<String, std::io::Error> {
    // Read the input.txt file provided by the Advent of Code website
    let input = fs::read_to_string(format!("src/{}/input.txt", day_number))?;

    // Trim the end of the file to ensure no \n will remain to screw you over later
    Ok(input.trim_end().to_string())
}
