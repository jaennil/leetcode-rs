fn main() {}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut cx = x;

    let mut len = len(x as u32);

    let mut reversed = 0;

    while cx > 0 {
        let last_digit = cx % 10;
        reversed += last_digit * 10_i32.pow(len - 1);
        cx /= 10;
        len -= 1;
    }

    return reversed == x;
}

fn len(x: u32) -> u32 {
    let mut cnt = 1;
    let mut divider: u64 = 10;
    while x as u64 / divider >= 1 {
        cnt += 1;
        divider *= 10;
    }
    return cnt;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_len1() {
        let input = 4;
        let expected = 1;
        let got = len(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_len2() {
        let input = 55;
        let expected = 2;
        let got = len(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_len3() {
        let input = 505;
        let expected = 3;
        let got = len(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_len4() {
        let input = 123;
        let expected = 3;
        let got = len(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_is_palindrome() {
        let input = 121;
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
