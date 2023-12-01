use std::fs;
fn main() {
    let file_contents = fs::read_to_string("./input/input.txt")
        .expect("LogRocket: Should have been able to read the file");

    let mut sum = 0;

    for line in file_contents.lines() {

        let mut charleft = "".to_string();
        let mut charright= "".to_string();

        for character in line.chars() {
            if character.is_digit(10) {
                charleft = character.to_digit(10).unwrap().to_string();
                break
            }
        }

        for character in line.chars().rev() {
            if character.is_digit(10) {
                charright = character.to_digit(10).unwrap().to_string();
                break
            }
        }

        sum = sum + {
            let number_string = charleft + &charright;
            number_string.parse::<i32>().unwrap()
        }
    }

    print!("{}", sum)
}