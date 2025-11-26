use rand::prelude::*;

const SPECIAL_CHARS: [char; 14] = [
    '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
];

type PickedChars = [char;4];

pub fn run(length: u8) -> String{
    if length < 4 {
        panic!("length should be greater than 3 to cover all kinds of chars");
    }

    let mut rng = rand::rng();

    let mut password = String::new();
    let mut is_valid_password = false;

    while !is_valid_password {
        generate_password(&mut password, &mut rng, length);
        is_valid_password = validate_password(&mut password);
    }

    password
}

pub fn generate_password(password: &mut String, rng: &mut ThreadRng, length: u8){
    password.clear();

    // initially we want to make sure that we have the 4 types of chars
    generate_initial_chars(password, rng);

    for _ in 4..length{
        let result = pick_chars(rng);

        let result = result
        .choose(rng).unwrap_or(&SPECIAL_CHARS[0]).clone();

        password.push(result);
    }
}

fn generate_initial_chars(password: &mut String, rng: &mut ThreadRng) {
    let mut result = pick_chars(rng);
    result.shuffle(rng);

    result.iter().for_each(|ch| password.push(ch.clone()));
}

fn pick_chars(rng: &mut ThreadRng) -> PickedChars {
    let numeric_char = ('0'..'9').choose(rng).unwrap_or_default();
    let low_alph_char = ('a'..'z').choose(rng).unwrap_or_default();
    let upp_alph_char = ('A'..'Z').choose(rng).unwrap_or_default();
    let symbol_alph_char = SPECIAL_CHARS.choose(rng).unwrap_or(&'$');

    [numeric_char, low_alph_char, upp_alph_char, symbol_alph_char.clone()]
}

pub fn validate_password(password: &String) -> bool{
    let low_case_check = password.chars().any(|ch| ('a'..'z').contains(&ch));
    let upp_case_check = password.chars().any(|ch| ('A'..'Z').contains(&ch));
    let num_check = password.chars().any(|ch| ('0'..'9').contains(&ch));
    let symbol_check = password.chars().any(|ch| SPECIAL_CHARS.contains(&ch));

    low_case_check && upp_case_check && num_check && symbol_check
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_validate_password(){
        let password = "Password123!";
        let result = validate_password(&String::from(password));
        assert!(result);
    }

    #[test]
    fn test_generate_password(){
        let mut password = String::new();
        let mut rng = rand::rng();
        let length = 10;
        generate_password(&mut password, &mut rng, length);
        assert!(validate_password(&password));
    }
}