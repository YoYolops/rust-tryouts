pub fn input() -> String {
    let mut input_string: String = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Panicked while colecting user input");

    input_string
}

pub fn parse_to_unsigned_int(str: &str) -> usize {
    let parsed_value: usize = str
        .trim()
        .parse()
        .expect("Panicked while parsing string");

    parsed_value
}

pub fn run() {
    let inputs_amount: usize = parse_to_unsigned_int(&input());
    let mut inputs: Vec<Input> = Vec::new();

    for i in 0..=(inputs_amount-1) {
        let user_input_string: String = input();

        let mut current_circle_size: usize = 0;
        let mut current_jump_size: usize = 0;

        let mut counter: usize = 0;
        for j in user_input_string.trim().split(" ") {
            match j.parse::<usize>() {
                Ok(number) => {
                    if counter == 0 {
                        current_circle_size = number
                    } else {
                        current_jump_size = number
                    }
                    counter += 1;
                },
                Err(_) => println!("Invalid numeric string"),
            }
        }

        inputs.insert(i, Input {
            jump_size: current_jump_size,
            circle_size: current_circle_size
        })
    }
    for i in 1..=inputs_amount {
        println!("Case {}: {}", i, run_killing_spree_v2(inputs[i-1].jump_size, inputs[i-1].circle_size));
    }
}

struct Input {
    jump_size: usize,
    circle_size: usize,
}

struct Person {
    alive: bool,
}

impl Person {
    fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false;
    }
}



fn create_circle(size: &usize) -> Vec<Person> {
    let mut circle = Vec::new();
    for _i in 0..=(size-1) {
        circle.push(Person {
            alive: true,
        })
    }
    circle
}

fn get_next_alive_position_from(position: usize, circle: &[Person]) -> usize {
    for i in position+1..=position+circle.len() {
        if circle[(i)%circle.len()].alive {
            return i%circle.len();
        }
    }
    panic!("Could not find the next alive");
}

fn find_next_target_position(current_killing_position: usize, jump_size: usize, circle: &Vec<Person>) -> usize {
    let mut current_guess: usize = current_killing_position;

    for _i in 1..=jump_size {
        current_guess = get_next_alive_position_from(current_guess, &circle);
    }
    current_guess
}

fn find_last_alive_position(circle: &[Person]) -> usize {
    for i in 0..=(circle.len()-1) {
        if circle[i].is_alive() {
            return i
        }
    }
    panic!("Could not find last alive");
}

fn run_killing_spree(jump_size: usize, circle_size: usize) -> usize {
    let mut dead_amount: usize = 0;
    let mut killing_position: usize = 0;
    let mut circle: Vec<Person> = create_circle(&circle_size);

    while dead_amount != (circle.len()-1) {
        killing_position = find_next_target_position(killing_position, jump_size, &circle);
        circle[killing_position].kill();
        dead_amount+=1;
    }
    find_last_alive_position(&circle)
}


fn run_killing_spree_v2(jump_size: usize, circle_size: usize) -> usize {
    // this version implements a MUCH simpler algorithm, runs faster, has fewer operations and uses less memory
    let mut people: Vec<usize> = Vec::new();

    for i in 0..=circle_size-1 {
        people.push(i);
    }

    let mut killed_amount: usize = 0;
    let mut current_index: usize = 0;

    while killed_amount != circle_size-1 {
        current_index = ((current_index + jump_size)-1)%people.len();
        people.remove(current_index);
        killed_amount += 1;
    }
    people[0]+1
}