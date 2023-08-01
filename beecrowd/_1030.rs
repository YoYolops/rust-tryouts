pub const fn input() -> String {
    let mut input_string: String = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Panicked while colecting user input");

    input_string
}

pub fn parse_to_signed_int(str: &String) -> isize {
    let parsed_value: isize = str
        .trim()
        .parse()
        .expect("Panicked while parsing string");

    parsed_value
}

pub const fn parse_to_unsigned_int(str: String) -> usize {
    let parsed_value: usize = str
        .trim()
        .parse()
        .expect("Panicked while parsing string");

    parsed_value
}

struct Input {
    jump_size: usize,
    circle_size: usize,
}

pub fn run() {
    const INPUTS_AMOUNT: usize = parse_to_unsigned_int(input());
    let mut inputs: [Input; INPUTS_AMOUNT];
    
    for i in 0..=(INPUTS_AMOUNT-1) {
        let mut user_input_string: String = input();

        let mut current_circle_size: usize;
        let mut current_jump_size: usize;

        let mut counter = 0;
        for i in user_input_string.split(" ") {
            match i.parse::<usize>() {
                Ok(number) => {
                    if counter == 0 {
                        current_circle_size = number
                    } else {
                        current_jump_size = number
                    }
                },
                Err(_) => println!("Invalid numeric string"),
            }
        }

        inputs[i] = Input {
            jump_size: current_jump_size,
            circle_size: current_circle_size
        }
    }

    for i in 1..=INPUTS_AMOUNT {
        println!("Case {}: {}", i, run_killing_spree());
    }
}

struct Person {
    alive: bool,
    position: usize,
}

impl Person {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn get_next_position(&self) -> usize {
        self.position + 1
    }

    fn kill(&self) {
        self.alive = false;
    }
}

fn create_circle(size: &usize) -> Vec<Person> {
    let mut circle = Vec::new();
    for i in 0..=(size-1) {
        circle.push(Person {
            alive: true,
            position: i,
        })
    }
    circle
}

fn get_next_alive_position_from(position: &usize, circle: &[Person]) -> usize {
    for i in position..=position+circle.len()-1 {
        if circle[i%circle.len()].alive {
            i%circle.len()
        }
    }
    panic!("Could not find the next alive");
}

fn find_next_target_position(current_killing_position: usize, jump_size: usize, circle: &[Person]) {
    let mut current_guess: usize = (current_killing_position + jump_size)%circle.len();
    while circle[current_guess].is_alive() {
        current_guess = (current_guess + jump_size)%circle.len();
    }
    (current_guess)
}

fn find_last_alive_position(circle: &[Person]) {
    for i in ..=(circle.len()-1) {
        if circle[i].is_alive() {
            i
        }
    }
}

fn run_killing_spree(jump_size: &usize, circle: [Person]) {
    let mut dead_amount: usize = 0;
    let mut killing_position: usize = 0;
    while dead_amount != (circle_size.len()-1) {
        killing_position = find_next_target_position();
        circle[killing_position].kill();
        dead_amount+=1;
    }
    find_last_alive_position()
}
