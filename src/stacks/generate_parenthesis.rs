use crate::starter::Solution;

impl Solution {
    //NOTE: Since we have to generate the possible combinations
    //of parenthesis, some may be valid and some not, we can leverage
    //backtracking. The idea is to recursively explore the various choices
    //and select only the valid one.
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let lefts = n;
        let rights = n;
        let mut combs = vec![];

        if n == 1 {
            return vec!["()".to_owned()];
        }

        Solution::backtrack(
            vec!['(', '('],
            "((".to_string(),
            lefts - 2,
            rights,
            &mut combs,
        );
        Solution::backtrack(vec![], "()".to_string(), lefts - 1, rights - 1, &mut combs);

        combs
    }

    //WARN: The backtracking needs to consider mutually exclusive scenarios
    //for each branch.
    pub fn backtrack(
        mut current_stack: Vec<char>,
        current_str: String,
        lefts: i32,
        rights: i32,
        combs: &mut Vec<String>,
    ) {
        if lefts == 0 && rights == 0 && current_stack.is_empty() {
            combs.push(current_str);
            return;
        }

        if lefts != 0 {
            let mut new_stack = current_stack.clone();
            new_stack.push('(');
            Solution::backtrack(
                new_stack,
                format!("{current_str}("),
                lefts - 1,
                rights,
                combs,
            );
        }

        if rights != 0 {
            if current_stack.is_empty() {
                return;
            }
            current_stack.pop();
            Solution::backtrack(
                current_stack,
                format!("{current_str})"),
                lefts,
                rights - 1,
                combs,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn test_generate_parenthesis_1() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"])
    }

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        )
    }

    #[test]
    fn test_generate_parenthesis_2() {
        assert_eq!(Solution::generate_parenthesis(2), vec!["()()", "(())"])
    }
}
