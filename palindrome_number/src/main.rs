fn main() {}

pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }

    let mut mut_x = x;

    let mut reversed = 0;

    while mut_x > 0 {
        let last_digit = mut_x % 10;
        reversed = reversed * 10 + last_digit;
        mut_x /= 10;
    }

    reversed == x
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_is_palindrome() {
        let input = 121;
        let expected = true;
        let got = is_palindrome(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_is_palindrome5() {
        let input = 424;
        let expected = true;
        let got = is_palindrome(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_is_palindrome2() {
        let input = -121;
        let expected = false;
        let got = is_palindrome(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_is_palindrome3() {
        let input = 10;
        let expected = false;
        let got = is_palindrome(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_is_palindrome4() {
        let input = 1410110141;
        let expected = true;
        let got = is_palindrome(input);
        assert_eq!(expected, got);
    }
}
