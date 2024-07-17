pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = haystack.len();
        let n = needle.len();

        if n > m {
            return -1;
        }

        for i in 0..=m-n {
            if &haystack[i..i+n] == needle {
                return i as i32;

            }
            
        }
        -1
       
}

fn main() {
   let haystack = "leetcode";
   let needle = "leeto";

   let res = str_str(haystack.to_owned(), needle.to_owned());

   println!("{:?}" , res);

   run_tests();
   
}

fn run_tests() {
    let test_cases = vec![
        ("hello".to_owned(), "ll".to_owned(), 2),
        ("aaaaa".to_owned(), "bba".to_owned(), -1),
        ("".to_owned(), "".to_owned(), 0),
        ("abc".to_owned(), "".to_owned(), 0),
        ("abc".to_owned(), "abc".to_owned(), 0),
        ("abc".to_owned(), "abcd".to_owned(), -1),
        ("abcabcabc".to_owned(), "abc".to_owned(), 0),
        ("mississippi".to_owned(), "issip".to_owned(), 4),
    ];

    for (haystack, needle, expected) in test_cases {
        let result = str_str(haystack.clone(), needle.clone());
        assert_eq!(result, expected, "Failed for haystack: {} and needle: {}", haystack, needle);
        println!("Test passed for haystack: '{}' and needle: '{}'", haystack, needle);
    }
}