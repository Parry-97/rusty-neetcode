use std::collections::HashSet;

use crate::starter::Solution;

impl Solution {
    ///
    /// NOTE: The recommended time complexity is `O(n^2)`. We could sort
    /// the input array to make the search faster and then apply
    /// two sum sorted while fixing one value of the three.
    /// The `sort_unstable` function has `O(nlogn)` worst case
    /// time complexity and does not allocate extra memory but
    /// change the ordering of equal elements.
    ///
    /// NOTE: If the output is not considered extra space, the current solution
    /// also uses only O(1) extra space.
    ///
    pub fn my_three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut left: usize;
        let mut right: usize;
        let mut i = 0;
        let mut triplet;

        let mut triplets = HashSet::new();

        while i < nums.len() {
            left = 0;
            right = nums.len() - 1;
            while left < right {
                if left == i {
                    left += 1;
                    continue;
                } else if right == i {
                    right -= 1;
                    continue;
                }

                match -nums[i] < nums[left] + nums[right] {
                    true => {
                        right -= 1;
                    }
                    false => match nums[left] + nums[right] < -nums[i] {
                        true => {
                            left += 1;
                        }
                        false => {
                            triplet = vec![nums[left], nums[right], nums[i]];
                            triplet.sort_unstable();
                            triplets.insert(triplet);
                            //WARN: Forgot that for the same number we could have more than
                            //one combination for the ith value
                            // i += 1; skipped values and failed for test case 101
                            // continue 'outer;
                            //
                            //The correct follow-up would be, as Neetcode suggests:
                            left += 1;
                            while nums[left] == nums[left - 1] && left < right {
                                left += 1;
                            }
                        }
                    },
                }
            }
            i += 1;
        }
        triplets.into_iter().collect()
    }

    ///NOTE: Neetcode implementation
    ///It sure is much faster and cleaner. It doesn't use sorting
    ///of triplets. I didn't consider the fact that for the ith value,
    ///there could be more combinations
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        nums.sort_unstable();

        for (i, a) in nums.iter().enumerate() {
            if i > 0 && *a == nums[i - 1] {
                continue;
            }

            let (mut left, mut right) = (i + 1, nums.len() - 1);
            while left < right {
                let threesum = nums[i] + nums[left] + nums[right];

                match threesum > 0 {
                    true => {
                        right -= 1;
                    }
                    false => match threesum < 0 {
                        true => {
                            left += 1;
                        }
                        false => {
                            res.push(vec![nums[left], nums[right], nums[i]]);
                            left += 1;
                            while nums[left] == nums[left - 1] && left < right {
                                left += 1;
                            }
                        }
                    },
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        )
    }

    #[test]
    fn check_three_sum_one() {
        let out: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), out);
    }

    #[test]
    fn check_three_sum_101() {
        assert_eq!(
            Solution::my_three_sum(vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4]),
            vec![
                [-4, 0, 4],
                [-4, 1, 3],
                [-3, -1, 4],
                [-3, 0, 3],
                [-3, 1, 2],
                [-2, -1, 3],
                [-2, 0, 2],
                [-1, -1, 2],
                [-1, 0, 1]
            ]
        );
    }
}
