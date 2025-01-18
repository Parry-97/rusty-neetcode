use crate::starter::Solution;

impl Solution {
    /// NOTE: The solution works but clearly is quite slow compared to other alternative.
    /// Nevertheless, first hard problem ever solved. Very brute force approach. The idea
    /// is to almost fill and cap the found enclosure so that it is no longer used in
    /// next iterations. The total water is accumulated and then returned
    pub fn my_trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut left = 0;
        let mut right;
        let mut temp_water = Vec::new();
        let mut total_water_content = 0;
        'outer: while left < height.len() {
            if height[left] > 0 {
                right = left + 1;
                temp_water.clear();
                while right < height.len() {
                    if height[right] >= height[left] {
                        total_water_content += temp_water
                            .iter()
                            .filter(|h| **h < height[right])
                            .map(|h| height[left] - *h)
                            .sum::<i32>();
                        left = right;
                        continue 'outer;
                    }
                    if height[right] > height[right - 1] {
                        let capped_water: i32 = temp_water
                            .iter()
                            .filter(|h| **h < height[right])
                            .map(|h| height[right] - *h)
                            .sum();
                        total_water_content += capped_water;
                        temp_water
                            .iter_mut()
                            .filter(|h| **h < height[right])
                            .for_each(|h| *h = height[right]);
                    }
                    temp_water.push(height[right]);
                    right += 1;
                }
                left = right;
            } else {
                left += 1;
            }
        }
        total_water_content
    }

    /// NOTE: Setting the brute force approach aside, we can use the hint from Neetcode
    /// of finding the max left and max right heights for each point and store it in arrays
    /// to then use that to compute the trapped water when
    /// possible. The time and space complexity would be O(n), which is still an improvement
    /// over the O(n^2) time complexity from brute force approaches.
    /// The approach suggested from Neetcode is indeed quite clean.
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let len = height.len();
        let mut left_max = vec![0; len];
        let mut right_max = vec![0; len];

        left_max[0] = height[0];
        for i in 1..len {
            left_max[i] = left_max[i - 1].max(height[i]);
        }

        right_max[len - 1] = height[len - 1];
        for i in (0..=len - 2).rev() {
            right_max[dbg!(i)] = right_max[i + 1].max(height[i]);
        }

        height
            .iter()
            .enumerate()
            .map(|(hi, h)| left_max[hi].min(right_max[hi]) - h)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_rain_water_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6)
    }

    #[test]
    fn check_rain_water_14() {
        assert_eq!(Solution::trap(vec![4, 2, 3]), 1)
    }

    #[test]
    fn check_rain_water_2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9)
    }
}
