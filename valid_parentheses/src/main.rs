pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            match char {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ if Some(char) != stack.pop() => return false,
                _ => (),
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let input = "()".to_string();
        let output = true;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test2() {
        let input = "()[]{}".to_string();
        let output = true;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test3() {
        let input = "(]".to_string();
        let output = false;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test4() {
        let input = "]".to_string();
        let output = false;
        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn test5() {
        let input = "(])".to_string();
        let output = false;
        assert_eq!(Solution::is_valid(input), output);
    }
}
