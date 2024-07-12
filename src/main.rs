pub fn reverse_vowels(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let vowels = |b: u8| matches!(b, b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U');
    let mut vb = s.into_bytes();
    let (mut i , mut j) = (0 , vb.len()-1);

    while i <j {
        while i < j && !vowels(vb[i]) {
            i += 1;
        }
        while i < j && !vowels(vb[j]){
            j-= 1;
        }
        if i < j {
            vb.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
    String::from_utf8(vb).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(reverse_vowels("hello".to_string()), "holle");
        assert_eq!(reverse_vowels("leetcode".to_string()), "leotcede");
        assert_eq!(reverse_vowels("aA".to_string()), "Aa");
        assert_eq!(reverse_vowels("".to_string()), "");
        assert_eq!(reverse_vowels("rust".to_string()), "rust");
    }
}

fn main() {
    let s = "hello".to_string();
    let res = reverse_vowels(s);
    println!("{:?}" , res);

}
