use crate::starter::Solution;

impl Solution {
    //NOTE: The approach I am gonna follow for this one is
    //similar to `MinStack`, in the sense that I keep track of all values
    //that are bigger than the current iterating value, I push them into a
    //stack and then pop them out only when a bigger value is found.
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut iter_stack: Vec<(usize, i32)> = vec![];

        for (i, temp) in temperatures.iter().enumerate() {
            while let Some((itop, elem)) = iter_stack.pop() {
                if dbg!(*temp) <= dbg!(elem) {
                    //make sure to push in the stack if equal values are
                    //found hence the <=
                    iter_stack.push((itop, elem));
                    break;
                } else {
                    res[itop] = dbg!(i - itop) as i32;
                }
            }
            iter_stack.push((i, *temp));
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_daily_temps_6() {
        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        )
    }
}
