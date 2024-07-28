use rand::Rng;
use std::io;

fn parse_int(mut input:&str) -> i32 {
    let parsed = input.trim().parse::<i32>().expect("Not a valid number");
    return parsed;
}

fn parse_bool(mut input:&str) -> bool {
    let parse_to_bool: fn(&str) -> bool = |x:&str| x.to_string() == "s";
    let parsed = parse_to_bool(&input);
    return parsed;
}

fn main() {
    let mut password_length = 0;
    let mut include_uppercase = false;
    let mut include_lowercase = false;
    let mut include_symbols = false;
    let mut include_numbers = false;
    println!("Insira o tamanho da senha");
    password_length = get_user_input(parse_int);
    println!("Inclui maiusculas? S/N");
    include_uppercase = get_user_input(parse_bool);
    println!("Inclui minusculas? S/N");
    include_lowercase = get_user_input(parse_bool);
    println!("Inclui simbolos? S/N");
    include_symbols = get_user_input(parse_bool);
    println!("Inclui numeros? S/N");
    include_numbers = get_user_input(parse_bool);

    let password = generate_password(password_length,
                                     include_uppercase,
                                     include_lowercase,
                                     include_symbols,
                                     include_numbers);
    println!("Generated password {}", password)
}

fn get_user_input<T>(parse:fn(&str)->T) -> T {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parameter = parse(input.trim());
    return parameter;
}

fn generate_password(password_length: i32,
                     include_uppercase: bool,
                     include_lowercase: bool,
                     include_symbols: bool,
                     incluide_numbers: bool) -> String {
    let mut rng = rand::thread_rng();
    let mut charset = String::new();

    if include_uppercase{
        charset += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    }
    if include_lowercase{
        charset += "abcdefghijklmnopqrstuvwxyz";
    }
    if include_symbols{
        charset += "0123456789";
    }
    if incluide_numbers{
        charset += "!@#$%^&*()_+-=[]{}|;:'\",.<>?/`~";
    }

    if charset.is_empty(){
        panic!("Ao menos uma letra deve ser selecionada");
    }

    (0..password_length)
        .map(|_|{
            let index = rng.gen_range(0..charset.len());
            charset.chars().nth(index).unwrap()
        })
        .collect()
}