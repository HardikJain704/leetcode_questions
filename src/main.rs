pub fn is_match(s: String, p: String) -> bool {
    let m = s.len();
    let n = p.len();

    let mut dp = vec![vec![false; n + 1]; m + 1];

    let sb = s.as_bytes();
    let pb = p.as_bytes();

    dp[m][n] = true;

    for i in (0..=m).rev() {
        for j in (0..=n).rev() {
            if j < n {
                if pb[j] == b'*' {
                    dp[i][j] = dp[i][j + 1];
                    if i < m {
                        dp[i][j] = dp[i][j] || dp[i + 1][j];
                    }
                } else {
                    if i < m && (sb[i] == pb[j] || pb[j] == b'?') {
                        dp[i][j] = dp[i][j] || dp[i + 1][j + 1];
                    }
                }
            }
        }
    }

    dp[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert_eq!(is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(is_match("aa".to_string(), "aa".to_string()), true);
        assert_eq!(is_match("cb".to_string(), "?a".to_string()), false);
        assert_eq!(is_match("adceb".to_string(), "*a*b".to_string()), true);
        assert_eq!(is_match("acdcb".to_string(), "a*c?b".to_string()), false);
        assert_eq!(is_match("".to_string(), "*".to_string()), true);
        assert_eq!(is_match("".to_string(), "a*".to_string()), false);
        assert_eq!(is_match("a".to_string(), "".to_string()), false);
    }
}

fn main() {
    let s = "cb".to_string();
    let p = "?a".to_string();

    let res = is_match(s, p);

    println!("{:?}", res);
}

