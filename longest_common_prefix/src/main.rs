pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mn_str = strs.iter().min().unwrap();

    for i in 0..mn_str.len() {
        for j in 1..strs.len() {
            if strs[j].chars().nth(i) != strs[j - 1].chars().nth(i) {
                return mn_str[..i].to_string();
            }
        }
    }

    mn_str.to_string()
}

#[cfg(test)]
mod tests {
    use crate::longest_common_prefix;

    #[test]
    fn lcp() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let expected = "fl";
        let got = longest_common_prefix(input);
        assert_eq!(expected, got);
    }

    #[test]
    fn lcp2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let expected = "";
        let got = longest_common_prefix(input);
        assert_eq!(expected, got);
    }
}
