pub fn run() {
    let mut evens_amount: i32 = 0;

    for _i in 0..=4 {
        let mut user_input: String = String::new();

        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Software panicked while reading user input");

        let parsed_user_input: i32 = user_input
            .trim()
            .parse()
            .expect("Panicked while parsing string user input to i32");



        if parsed_user_input % 2 == 0 {
            evens_amount += 1;
        }
    }

    println!("{} valores pares", evens_amount);
}