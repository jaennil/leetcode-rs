pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    Vec::new()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let nums = [2, 7, 11, 15];
        let target = 9;
        let got = two_sum(nums.to_vec(), target);
        let expected = [0, 1];
        assert_eq!(got, expected);
    }

    #[test]
    fn test2() {
        let nums = [3, 2, 4];
        let target = 6;
        let got = two_sum(nums.to_vec(), target);
        let expected = [1, 2];
        assert_eq!(got, expected);
    }

    #[test]
    fn test3() {
        let nums = [3, 3];
        let target = 6;
        let got = two_sum(nums.to_vec(), target);
        let expected = [0, 1];
        assert_eq!(got, expected);
    }

    #[test]
    fn test4() {
        let nums = [3, 2, 3];
        let target = 6;
        let got = two_sum(nums.to_vec(), target);
        let expected = [0, 2];
        assert_eq!(got, expected);
    }
}

fn main() {}
