// 1066 - Pares, Ímpares, Positivos e Negativos - 
pub fn main() {
    let mut evens: i32 = 0;
    let mut odds: i32 = 0;
    let mut negatives: i32 = 0;
    let mut positives: i32 = 0;

    for _i in 1..=5 {
        let mut user_input = String::new();
        
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let int_user_input: i32 = user_input
            .trim()
            .parse()
            .expect("Wanted a number");

        if int_user_input < 0 {
            *&mut negatives += 1;
        } else if int_user_input > 0 {
            *&mut positives += 1;
        }

        if (int_user_input % 2) == 0 {
            *&mut evens += 1;
        } else {
            *&mut odds += 1;
        }
    }

    println!("{} valor(es) par(es)", evens);
    println!("{} valor(es) impar(es)", odds);
    println!("{} valor(es) positivo(s)", positives);
    println!("{} valor(es) negativo(s)", negatives);
}