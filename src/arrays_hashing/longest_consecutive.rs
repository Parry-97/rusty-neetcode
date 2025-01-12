use std::collections::HashSet;

use crate::starter::Solution;

impl Solution {
    //NOTE: This is reminescent of two sum but repeatead.
    //We can use a hashset to iteratively find ,starting from the first,
    //whether a sequence can be constructed by looking at whether the n-1 value
    //exists in the hashset.

    //WARN: Leetcode runtime warns me of a 'Time Limit Exceeded' error especially
    //for very large input test cases. I think the solution
    //is actually correct.
    pub fn my_longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        if nums.is_empty() {
            return 0;
        }
        let mut numset = HashSet::new();
        nums.iter().for_each(|n| {
            numset.insert(n);
        });
        for n in nums.iter() {
            let mut current_val = *n;
            let mut current_len = 1;
            while numset.contains(&(current_val - 1)) {
                current_len += 1;
                current_val -= 1;
            }
            max_len = max_len.max(current_len);
        }
        max_len
    }

    //NOTE: The version from Neetcode iterates over the hashset instead of the input
    //list like I do in my attempt, possibly reducing the set
    //of input values in nums and avoiding the time limit error due to duplicates.
    //Furthemore, the conditional statement in the outer loop helps us detect the
    //start of each sequence. We then check for the subsequent values in the sequence.
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max_len = 1;
        if nums.is_empty() {
            return 0;
        }
        let mut numset = HashSet::new();
        nums.iter().for_each(|n| {
            numset.insert(n);
        });
        for n in numset.iter() {
            let mut current_val = **n;
            let mut current_len = 1;
            if !numset.contains(&(current_val - 1)) {
                while numset.contains(&(current_val + 1)) {
                    current_len += 1;
                    current_val += 1;
                }
                max_len = max_len.max(current_len);
            }
        }
        max_len
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_long_sequence() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4)
    }
}
