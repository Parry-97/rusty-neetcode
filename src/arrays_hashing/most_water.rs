use crate::starter::Solution;

impl Solution {
    ///NOTE: A very straightforward solution for this would be to
    ///use of course the two pointers approach, have them start at the
    ///beginning and the end and visit the various combinations and
    ///increasing the pointer that in the iteration provides less value,
    ///like an optimization problem.
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;

        let mut max_water = 0;
        let mut water;

        while left < right {
            water = height[left].min(height[right]) * (right - left) as i32;
            max_water = max_water.max(water);

            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        max_water
    }
}
