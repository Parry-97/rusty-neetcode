use crate::starter::Solution;

impl Solution {
    ///NOTE: If Leetcode allowed for a mutable reference to the vector of
    ///tokens, instead of an owned vector, this is the Solution I would opt
    ///for. It uses the tokens as a stack and then `recursively`
    ///compute operands and expressions.
    ///Time Complexity is O(n). Not so sure about the space complexity since
    ///I not really cloning the tokens vector, hence O(1) ? Please correct
    ///If I'm wrong. Of course the activation frames surely impact the
    ///memory usage.
    ///
    ///NOTE: I already had exposure to RPN in high school, therefore the concept
    ///was very familiar to me.
    ///Based on this we can easily create an iterative version as well, but I really like
    ///the recursive one
    pub fn recursive_eval_rpn(tokens: &mut Vec<String>) -> i32 {
        let operator = tokens.pop().unwrap();

        if let Ok(n) = operator.parse::<i32>() {
            return n;
        }
        let op2 = tokens.pop().unwrap();
        let op2 = match dbg!(op2.as_str()) {
            "+" | "-" | "/" | "*" => {
                tokens.push(op2);
                Solution::recursive_eval_rpn(tokens)
            }
            _ => op2.parse::<i32>().unwrap(),
        };

        let op1 = tokens.pop().unwrap();
        let op1 = match dbg!(op1.as_str()) {
            "+" | "-" | "/" | "*" => {
                tokens.push(op1);
                Solution::recursive_eval_rpn(tokens)
            }
            _ => op1.parse::<i32>().unwrap(),
        };

        let res = match operator.as_str() {
            "+" => op1 + op2,
            "-" => op1 - op2,
            "/" => op1 / op2,
            "*" => op1 * op2,
            _ => operator.parse().unwrap(),
        };
        dbg!(res)
    }

    ///NOTE: This is also a nice solution from Neetcode where we use
    ///a support stack where we push all operands and results of intermediate
    ///expressions. These are then used as operands whenever an operator is found
    ///in the original stack.
    pub fn iterative_eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens.iter() {
            match token.as_str() {
                "+" => {
                    let second_op = stack.pop().unwrap();
                    let first_op = stack.pop().unwrap();
                    stack.push(first_op + second_op);
                }

                "-" => {
                    let second_op = stack.pop().unwrap();
                    let first_op = stack.pop().unwrap();
                    stack.push(first_op - second_op);
                }
                "*" => {
                    let second_op = stack.pop().unwrap();
                    let first_op = stack.pop().unwrap();
                    stack.push(first_op * second_op);
                }
                "/" => {
                    let second_op = stack.pop().unwrap();
                    let first_op = stack.pop().unwrap();
                    stack.push(first_op / second_op);
                }
                num => stack.push(num.parse::<i32>().unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::starter::Solution;

    #[test]
    fn check_recursive_rpn_1() {
        assert_eq!(
            Solution::recursive_eval_rpn(&mut vec![
                "2".to_owned(),
                "1".to_owned(),
                "+".to_owned(),
                "3".to_owned(),
                "*".to_owned()
            ]),
            9
        )
    }

    #[test]
    fn check_recursive_rpn_2() {
        assert_eq!(
            Solution::recursive_eval_rpn(&mut vec![
                "4".to_owned(),
                "13".to_owned(),
                "5".to_owned(),
                "/".to_owned(),
                "+".to_owned()
            ]),
            6
        );
    }

    #[test]
    fn check_recursive_rpn_3() {
        assert_eq!(
            Solution::recursive_eval_rpn(&mut vec![
                "10".to_owned(),
                "6".to_owned(),
                "9".to_owned(),
                "3".to_owned(),
                "+".to_owned(),
                "-11".to_owned(),
                "*".to_owned(),
                "/".to_owned(),
                "*".to_owned(),
                "17".to_owned(),
                "+".to_owned(),
                "5".to_owned(),
                "+".to_owned()
            ]),
            22
        );
    }
}
