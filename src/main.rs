pub fn reverse_str(s: String, k: i32) -> String {
        if k <= 0 {
            return s;
        }

        let a = k as usize;
        let mut b = s.chars().collect::<Vec<_>>();

        for start in (0..s.len()).step_by(2 * a){
            let mut i = start;
            let mut j =  (start + a - 1).min(s.len() - 1);

            while i < j {
                b.swap(i , j);
                i += 1;
                j -= 1;
            }


        }

        b.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let s = "abcdefg".to_string();
        let k = 2;
        let result = reverse_str(s, k);
        assert_eq!(result, "bacdfeg");
    }

    #[test]
    fn test_example2() {
        let s = "abcd".to_string();
        let k = 4;
        let result = reverse_str(s, k);
        assert_eq!(result, "dcba");
    }

    #[test]
    fn test_odd_length() {
        let s = "abcdef".to_string();
        let k = 3;
        let result = reverse_str(s, k);
        assert_eq!(result, "cbadef");
    }

    #[test]
    fn test_even_length() {
        let s = "abcdefgh".to_string();
        let k = 2;
        let result = reverse_str(s, k);
        assert_eq!(result, "bacdfegh");
    }

    #[test]
    fn test_single_character() {
        let s = "a".to_string();
        let k = 1;
        let result = reverse_str(s, k);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_k_greater_than_length() {
        let s = "abc".to_string();
        let k = 5;
        let result = reverse_str(s, k);
        assert_eq!(result, "cba");
    }

    #[test]
    fn test_k_equals_zero() {
        let s = "abcde".to_string();
        let k = 0;
        let result = reverse_str(s, k);
        assert_eq!(result, "abcde");
    }
}

fn main() {
   let s = "abcd";
   let k = 3;
   let res = reverse_str(s.to_owned(), k);

   println!("{:?}" , res);
   
}
