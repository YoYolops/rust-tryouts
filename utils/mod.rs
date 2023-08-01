pub fn input() -> String {
    let mut input_string: String = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Panicked while colecting user input");

    input_string
}

pub fn parse_to_int(str: &String) -> isize {
    let parsed_value: isize = str
        .trim()
        .parse()
        .expect("Panicked while parsing string");

    parsed_value
}

pub fn parse_to_float(str: &String) -> f64 {
    let parsed_value: f64 = str
        .trim()
        .parse()
        .expect("Panicked while parsing string to f64");

    parsed_value
}