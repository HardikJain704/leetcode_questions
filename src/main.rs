// In this approach, I am pushing romans to hash map and then iterating over the 
// string from end.
use std::collections::HashMap;
pub fn roman_to_int(s: String) -> i32 {
    let mut map = HashMap::new();

    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;
    let mut prev_value = 0;

    for &ch in chars.iter().rev() {
        if let Some(&value) = map.get(&ch) {
            if value < prev_value {
                result -= value;
            } else {
                result += value;
            }
            prev_value = value;
        }
    }

    result
}

fn main() {
    let roman = String::from("IX");
    let integer = roman_to_int(roman);
    println!("{:?}", integer);
}
