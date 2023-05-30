fn main() {
    let base_string = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();

    println!("{}", convert_from_base_ten(19, 20, base_string));
    println!("{}", convert_to_base_ten(String::from("235"), 8, base_string));
    println!("{}", convert_from_base_to_base(String::from("235"), 8, 16, base_string));
}

// function for any base to any other base. potential function: https://stackoverflow.com/questions/3973685/python-homework-converting-any-base-to-any-base

fn convert_from_base_ten(number: u32, base: u32, base_key: &str) -> String {
    let mut divisor = base;
    let mut output = String::from("");
    let mut place_values = 1;
    let mut pre_output = number as f32;
    while number / divisor >= 1 {
        place_values += 1;
        divisor *= base;
    }
    divisor /= base;
    pre_output /= divisor as f32;
    let num = pre_output.floor() as usize;
    output += &base_key[num..num + 1];

    while place_values > 1 {
        let num = pre_output.floor();
        pre_output -= num;
        pre_output *= base as f32;
        let num = pre_output.floor() as usize;
        output += &base_key[num..num + 1];
        place_values -= 1;
    }

    output
}

// convert to base 10 https://mathbits.com/MathBits/CompSci/Introduction/tobase10.htm

//TODO: fix
fn convert_to_base_ten(number: String, base: u32, base_key: &str) -> u32 {
    let mut multiplier = base.pow(number.len() as u32);
    let mut output: u32 = 0;

    multiplier /= base;

    for character in number.chars() {
        let num = base_key.find(character).unwrap();
        output += num as u32 * multiplier;
        multiplier /= base;
    }

    output
}

fn convert_from_base_to_base(number: String, base_in: u32, base_out: u32, base_key: &str) -> String {
    convert_from_base_ten(convert_to_base_ten(number, base_in, base_key), base_out, base_key)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_convert_from_base_ten() {
        let base_key = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_ten(100, 8, base_key), String::from("144"));
    }

    #[test]
    fn test_convert_to_base_ten() {
        let base_key = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_to_base_ten(String::from("144"), 8, base_key), 100);
    }

    #[test]
    fn test_convert_from_base_to_base() {
        let base_key = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_to_base(String::from("144"), 8, 16, base_key), String::from("64"));
    }

    #[test]
    fn test_convert_from_base_ten_with_letter() {
        let base_key = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_ten(255, 16, base_key), String::from("FF"));
    }

    #[test]
    fn test_convert_from_base_to_base_with_letter() {
        let base_key = "0123456789ABCDEFGHJKLMNOPQRSTUVWXYZ".trim();
        assert_eq!(convert_from_base_to_base(String::from("FF"), 16, 20, base_key), String::from("CF"));
    }
}
