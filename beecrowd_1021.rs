pub fn run() {
    let mut user_input: String = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Software panicked while reading user input");

    let raw_value: f64 = user_input
        .trim()
        .parse()
        .expect("Inputed value to be a number");

    let mut current_value: f64 = raw_value;

    let bills: [f64; 6] = [100.00, 50.00, 20.00, 10.00, 5.00, 2.00];
    let mut bills_amount: [u8; 6] = [0; 6];

    let coins: [f64; 6] = [1.00, 0.50, 0.25, 0.10, 0.05, 0.01];
    let mut coins_amount: [u8; 6] = [0; 6];

    for i in 0..bills.len() {
        bills_amount[i] = ((current_value - (current_value % bills[i]))/bills[i]) as u8;
        current_value = current_value % bills[i];
    }

    for i in 0..coins.len() {
        coins_amount[i] = ((current_value - (current_value % coins[i]))/coins[i]) as u8;
        current_value = current_value % coins[i];
    }

    println!("NOTAS:");
    println!("{} nota(s) de R$ 100.00", bills_amount[0]);
    println!("{} nota(s) de R$ 50.00", bills_amount[1]);
    println!("{} nota(s) de R$ 25.00", bills_amount[2]);
    println!("{} nota(s) de R$ 10.00", bills_amount[3]);
    println!("{} nota(s) de R$ 5.00", bills_amount[4]);
    println!("{} nota(s) de R$ 2.00", bills_amount[5]);
    println!("MOEDAS:");
    println!("{} moeda(s) de R$ 1.00", coins_amount[0]);
    println!("{} moeda(s) de R$ 0.50", coins_amount[1]);
    println!("{} moeda(s) de R$ 0.25", coins_amount[2]);
    println!("{} moeda(s) de R$ 0.10", coins_amount[3]);
    println!("{} moeda(s) de R$ 0.05", coins_amount[4]);
    println!("{} moeda(s) de R$ 0.01", coins_amount[5]);
}