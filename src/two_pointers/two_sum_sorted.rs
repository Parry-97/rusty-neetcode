use crate::starter::Solution;

//NOTE: I know two pointers approach should be used, but the only thing
//I think of is binary search as soon as I heard 'sorted'.

//WARN: The solution must only use O(1) additional space, therefore
//no hashmap can be used like with Two Sum I.
impl Solution {
    pub fn two_sum_sorted(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        if numbers[left] + numbers[right] == target {
            return vec![left as i32 + 1, right as i32 + 1];
        }

        let mut sum: i32;
        while left < right {
            sum = numbers[left] + numbers[right];
            if sum < target {
                left += 1;
            } else {
                match sum > target {
                    true => {
                        right -= 1;
                    }
                    false => break,
                }
            }
        }
        vec![left as i32 + 1, right as i32 + 1]
    }
}
