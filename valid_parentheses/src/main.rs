fn main() {}

const OPEN_BRACKETS: [char; 3] = ['(', '[', '{'];
const CLOSE_BRACKETS: [char; 3] = [')', ']', '}'];

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for char in s.chars() {
            if OPEN_BRACKETS.contains(&char) {
                stack.push(char);
            } else if let Some(&last_bracket) = stack.last() {
                let open_index = Self::index(OPEN_BRACKETS.to_vec(), last_bracket);
                let close_index = Self::index(CLOSE_BRACKETS.to_vec(), char);
                if open_index == close_index {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        stack.is_empty()
    }

    fn index(array: Vec<char>, value: char) -> usize {
        array.iter().position(|&x| x == value).unwrap()
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
