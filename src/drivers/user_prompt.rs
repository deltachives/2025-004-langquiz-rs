use std::{io::stdin, str::FromStr};

pub fn read_str_or_quit(item_name: &str) -> Option<String> {
    let mut mut_input = String::new();

    println!("Enter {item_name} or type [q]uit: ");
    stdin().read_line(&mut mut_input).unwrap();

    mut_input = mut_input.trim().to_owned();

    if mut_input == "quit" || mut_input == "q" {
        return None;
    }

    Some(mut_input)
}

pub fn read_input_from_user_until_valid_or_quit<T: FromStr>(item_name: &str) -> Option<T> {
    let mut mut_input;

    loop {
        mut_input = "".to_owned();

        println!("Enter {} or type [q]uit: ", item_name);
        stdin().read_line(&mut mut_input).unwrap();

        mut_input = mut_input.trim().to_owned();

        if mut_input == "quit" || mut_input == "q" {
            break None;
        }

        let parsed = T::from_str(&mut_input);

        match parsed {
            Ok(item) => break Some(item),
            Err(_) => {
                continue;
            }
        }
    }
}

pub fn read_binary_choice_from_user(prompt: &str) -> bool {
    let mut mut_input;

    loop {
        mut_input = "".to_owned();

        println!("{prompt}? ([y]es/[n]o)");
        stdin().read_line(&mut mut_input).unwrap();

        mut_input = mut_input.trim().to_owned().to_lowercase();

        if mut_input == "y" || mut_input == "yes" {
            return true;
        }

        if mut_input == "n" || mut_input == "no" {
            return false;
        }
    }
}
