use crate::starter::Solution;

//NOTE: I may have probably done this problem a couple of times so the solution
//is now well understood. We are basically using two vectors of size n each, to
//collect all the right products and the left products relative to an element
//in the input vector nums
//We are then using them to then calculate the product of all the left elements
//and all the rights elements relative to an element in the nums vector.
//This solution has time and space complexity O(n).
impl Solution {
    pub fn my_product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let lefts = nums
            .iter()
            .fold((1, vec![]), |(mut acc, mut l), e| {
                acc *= e;
                l.push(acc);
                (acc, l)
            })
            .1;

        let mut rights = nums
            .iter()
            .rev()
            .fold((1, vec![]), |(mut acc, mut r), e| {
                acc *= e;
                r.push(acc);
                (acc, r)
            })
            .1;

        rights.reverse();
        (0..nums.len())
            .map(|i| {
                if i == 0 {
                    rights[i + 1]
                } else if i == nums.len() - 1 {
                    lefts[i - 1]
                } else {
                    rights[i + 1] * lefts[i - 1]
                }
            })
            .collect()
    }

    //NOTE: In this follow-up we will try to use O(1) space complexity as shown in the optimal
    //solution from Neetcode.
    //To do so, we are gonna use the same idea but remember that the output array does not
    //count as extra space and therefore we are gonna use that to store our intermediate
    //results during our two passes of the input array. The only other extra space comes
    //from the fix variable, which is an accumulator for the postfix and the prefix
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut fix = 1;
        let mut outs = nums.iter().fold(vec![], |mut l, e| {
            l.push(fix);
            fix *= e;
            l
        });

        fix = 1;
        outs = nums
            .iter()
            // .enumerate()
            .rev()
            .zip(outs.iter().rev())
            .map(|(n, out)| {
                let newout = out * fix;
                fix *= n;
                newout
            })
            .collect();
        outs.reverse();
        outs
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_product() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            [24, 12, 8, 6]
        )
    }
}
