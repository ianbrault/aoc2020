/*
** src/puzzle/day18.rs
** https://adventofcode.com/2020/day/18
*/

use crate::puzzle::*;
use crate::utils::input_to_lines;

const INPUT: &str = include_str!("../../input/18.input");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(u64),
    OpAdd,
    OpMul,
    LParen,
    RParen,
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '+' => Self::OpAdd,
            '*' => Self::OpMul,
            '(' => Self::LParen,
            ')' => Self::RParen,
            _ => Self::Number(c.to_digit(10).unwrap() as u64),
        }
    }
}

struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    fn parse_token_stream(s: &'static str) -> Vec<Token> {
        s.chars().filter(|&c| c != ' ').map(Token::from).collect()
    }

    fn into_rpn(tokens: Vec<Token>, add_prec: u8, mul_prec: u8) -> Vec<Token> {
        // an implementation of the shunting-yard algorithm
        // converts the token stream into reverse-Polish notation
        let mut output = Vec::with_capacity(tokens.len());
        let mut op_stack = Vec::with_capacity(tokens.len());

        let op_prec = |op| match op {
            Token::OpAdd => add_prec,
            Token::OpMul => mul_prec,
            _ => panic!("invalid operator {:?}", op),
        };

        for token in tokens.into_iter() {
            match token {
                // push the number to the output queue
                Token::Number(_) => output.push(token),
                // while the top of the operator stack is not a left parenthesis
                // and has a greater precedence than the current operator, pop
                // from the operator stack onto the output queue; then push the
                // operator to the operator stack
                Token::OpAdd | Token::OpMul => {
                    while op_stack.last().is_some()
                        && *op_stack.last().unwrap() != Token::LParen
                        && op_prec(*op_stack.last().unwrap()) >= op_prec(token)
                    {
                        output.push(op_stack.pop().unwrap());
                    }
                    op_stack.push(token);
                }
                // push the lef parenthesis onto the operator stack
                Token::LParen => op_stack.push(token),
                Token::RParen => {
                    // pop operators onto the output queue while the top of the
                    // operator stack is not a left parenthesis
                    while *op_stack.last().unwrap() != Token::LParen {
                        output.push(op_stack.pop().unwrap());
                    }
                    // pop the left parenthesis
                    op_stack.pop();
                }
            }
        }

        // pop remaining operators onto the output queue
        while !op_stack.is_empty() {
            output.push(op_stack.pop().unwrap());
        }

        output
    }

    fn parse(s: &'static str, add_prec: u8, mul_prec: u8) -> Self {
        let tokens = Self::parse_token_stream(s);
        Self {
            tokens: Self::into_rpn(tokens, add_prec, mul_prec),
        }
    }

    fn evaluate(&self) -> u64 {
        let mut operand_stack = Vec::with_capacity(self.tokens.len());

        for token in self.tokens.iter() {
            match token {
                // add operands to the operand stack
                Token::Number(x) => operand_stack.push(*x),
                // pop operands and evaluate
                Token::OpAdd => {
                    let op_a = operand_stack.pop().unwrap();
                    let op_b = operand_stack.pop().unwrap();
                    operand_stack.push(op_a + op_b);
                }
                // pop operands and evaluate
                Token::OpMul => {
                    let op_a = operand_stack.pop().unwrap();
                    let op_b = operand_stack.pop().unwrap();
                    operand_stack.push(op_a * op_b);
                }
                _ => panic!("invalid token {:?}", token),
            }
        }

        operand_stack.pop().unwrap()
    }
}

pub struct Day18 {}

impl Day18 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Puzzle for Day18 {
    // Evaluate the expression on each line of the homework; what is the sum of
    // the resulting values?
    fn part1(&self) -> Result<Solution> {
        let sum = input_to_lines(INPUT)
            .map(|line| Expression::parse(line, 1, 1))
            .map(|expr| expr.evaluate())
            .sum::<u64>();
        Ok(sum.into())
    }

    // What do you get if you add up the results of evaluating the homework
    // problems when addition has higher precedence than multiplication?
    fn part2(&self) -> Result<Solution> {
        let sum = input_to_lines(INPUT)
            .map(|line| Expression::parse(line, 2, 1))
            .map(|expr| expr.evaluate())
            .sum::<u64>();
        Ok(sum.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate(s: &'static str, a: u8, m: u8) -> u64 {
        Expression::parse(s, a, m).evaluate()
    }

    #[test]
    fn expressions_equal_precedence() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6", 1, 1), 71);
        assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))", 1, 1), 51);
        assert_eq!(evaluate("2 * 3 + (4 * 5)", 1, 1), 26);
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1, 1), 437);
        assert_eq!(
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 1, 1),
            12240
        );
        assert_eq!(
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 1, 1),
            13632
        );
    }
}
