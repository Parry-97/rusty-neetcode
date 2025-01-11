use crate::starter::Solution;

//NOTE: the Solution presented here are based on hints provided by Neetcode.io.
//
//NOTE: Since the strings in the strs parameter can be composed of any UTF-8 character,
//we can't just use a delimiter character approach.
//We can use an encoding approach where we start with a number representing the length of
//the string, followed by a separator character (let's use # for simplicity), and then the
//string itself. To decode, we read the number until we reach a #, then use that number to
//read the specified number of characters as the string.
//
//NOTE: Doing so we are 'augmenting' the original strings with extra 'metadata' that turn out to be
//useful for the decoding phase, similar to how some common encodings work.

//WARN: This is a premium problem (which I am too poor to access). In case you find any issues
//with the solution please let me know.
impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        strs.iter()
            .map(|s| format!("{}#{s}", s.len()))
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut final_strs = vec![];
        let s = s.chars().collect::<Vec<char>>();
        let mut i = 0;
        while i < s.len() {
            let mut j = i;
            while s[j] != '#' {
                j += 1;
            }

            let len = s.get(i..j).unwrap();
            let len = String::from_iter(len.iter()).parse::<usize>().unwrap();
            i += 1;
            j = i + len;

            final_strs.push(String::from_iter(s.get(i..j).unwrap()));
        }

        final_strs
    }
}
