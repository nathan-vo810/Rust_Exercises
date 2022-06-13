use regex::Regex;

fn main() {
    println!("Enter the input string: ");
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .unwrap();

    println!("Enter string to search: ");
    let mut search_string = String::new();
    std::io::stdin()
        .read_line(&mut search_string)
        .unwrap();
    search_string.pop(); // Trim new line character at the end

    let pattern = format!(r"(?i:{})", search_string);
    let re = Regex::new(pattern.as_str()).unwrap();
    let count = re.captures_iter(&mut input_string).count();

    println!("Number of occurences of {} : {}", search_string, count);
}
