use std::collections::HashMap;

use crate::starter::Solution;

//TODO: Currently the best achieved runtime is 6ms. Need to look for Neetcode Solution
//to find some optmizations. We can't do ascii maths like maybe in C or other languages.
impl Solution {
    //
    //NOTE: Using a frequency hashmap to account for the frequency
    //of characters and use that as a key to then collect group of anagrams
    //as the values in the hashmap itself.
    pub fn my_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let alphabet = 'a'..='z';
        let alphabet = alphabet.collect::<Vec<char>>();

        let mut frequency_map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        strs.iter().for_each(|s| {
            let mut s_map = [0; 26];
            s.chars().for_each(|c| {
                s_map[alphabet.iter().position(|ab| *ab == c).unwrap()] =
                    s_map[alphabet.iter().position(|ab| *ab == c).unwrap()] + 1
            });

            frequency_map
                .entry(s_map)
                .and_modify(|e| e.push(s.clone()))
                .or_insert(vec![s.clone()]);
        });

        frequency_map.into_values().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn group_anagrams_first_test() {
        let strs = vec![
            "cat".to_owned(),
            "tac".to_owned(),
            "hat".to_owned(),
            "tah".to_owned(),
        ];
        assert_eq!(
            Solution::my_group_anagrams(strs),
            vec![
                ["cat".to_owned(), "tac".to_owned()],
                ["hat".to_owned(), "tah".to_owned()]
            ]
        );
    }
}
