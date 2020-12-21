use std::fs;

fn main() {
    // Read the passwords from a file
    let contents = fs::read_to_string("./passwords.txt").expect("Could not read file.");

    let mut valid_password = 0; 

    for password_and_rule in contents.lines() {
        let rules_and_password: Vec<&str> = password_and_rule.split(' ').collect();
        
        
        let char_nums: Vec<&str> = rules_and_password[0].split('-').collect();
        let (min_num, max_num) = (char_nums[0], char_nums[1]);
        let min_num = min_num.parse::<usize>().unwrap();
        let max_num = max_num.parse::<usize>().unwrap();
        let ruled_char_with_sep = rules_and_password[1];
        let ruled_char = ruled_char_with_sep.chars().nth(0).unwrap();
        let password = rules_and_password[2];

        let v: Vec<&str> = password.matches(ruled_char).collect();
        let num_of_ruled_char = v.len();

        if num_of_ruled_char >= min_num && num_of_ruled_char <= max_num {
            valid_password += 1;
        }
    }

    println!("{}", valid_password);

    // Part two!
    let mut valid_password_part_two = 0; 

    for password_and_rule in contents.lines() {
        let rules_and_password: Vec<&str> = password_and_rule.split(' ').collect();
        
        let char_nums: Vec<&str> = rules_and_password[0].split('-').collect();
        let (min_num, max_num) = (char_nums[0], char_nums[1]);
        let pos1 = min_num.parse::<usize>().unwrap();
        let pos2 = max_num.parse::<usize>().unwrap();
        let ruled_char_with_sep = rules_and_password[1];
        let ruled_char = ruled_char_with_sep.chars().nth(0).unwrap();
        let password = rules_and_password[2];

        let char_at_pos1 = password.chars().nth(pos1-1).unwrap();
        let char_at_pos2 = password.chars().nth(pos2-1).unwrap();

        if (char_at_pos1 == ruled_char && char_at_pos2 != ruled_char) ||
           (char_at_pos1 != ruled_char && char_at_pos2 == ruled_char) {
            valid_password_part_two += 1;
        }
    }
    println!("{}", valid_password_part_two);
}
