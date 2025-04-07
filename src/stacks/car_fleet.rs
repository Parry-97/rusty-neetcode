use crate::starter::Solution;

impl Solution {
    // WARN: My incorrect solution where I tried to simply merge the position after ordering, only
    // based on speed. Unfortunately this does not take into consideration the case when cars
    // arrive at the target at the same time
    pub fn my_car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ordered_position: Vec<(usize, i32)> = position.into_iter().enumerate().collect();
        ordered_position.sort_by_key(|x| x.1);
        let mut fleets: Vec<(usize, i32)> = vec![];

        ordered_position.into_iter().rev().for_each(|(si, p)| {
            if fleets.is_empty() {
                fleets.push((si, p));
            } else {
                let top = fleets.last().unwrap();
                if speed[top.0] > speed[si] {
                    fleets.push((si, p));
                }
            }
        });

        fleets.len() as i32
    }

    //INFO: Revised working solution based on Neetcode's suggestion where times are calculated
    //based on the initial offset and then the fleets are constructed based on the simple logic
    //that if the time of the car in front is higher than the car behind than they will form a
    //fleet
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ordered_position: Vec<(usize, i32)> = position.into_iter().enumerate().collect();
        ordered_position.sort_by_key(|x| x.1);

        let times = ordered_position
            .into_iter()
            .map(|x| ((target - x.1) as f32 / (speed[x.0] as f32)))
            .collect::<Vec<_>>();
        let mut fleets = vec![];
        dbg!(times).into_iter().rev().for_each(|x| {
            if fleets.is_empty() {
                fleets.push(x);
            } else {
                let top = fleets.last().unwrap();
                if *top < x {
                    fleets.push(x);
                }
            }
        });

        fleets.len() as i32
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_test_11() {
        assert_eq!(Solution::car_fleet(10, vec![6, 8], vec![3, 2]), 2)
    }
}
