mod utils;

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

impl Number {
    pub fn new(s: &str) -> Self {
        Self(s.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    pub fn new(op: &str) -> Self {
        match op {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => panic!("Bad Operator"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    lhs: Number,
    rhs: Number,
    op: Op,
}

impl Expr {
    pub fn new(expr: &str) -> Self {
        let (oprhs, lhs) = utils::extract_digits(&expr);
        let lhs = Number::new(lhs);
        let (rhs, op) = utils::extract_ops(&oprhs);
        let rhs = Number::new(rhs);
        let op = Op::new(op);

        Expr { lhs, rhs, op }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        assert_eq!(Number::new("123"), Number(123));
    }

    #[test]
    fn parse_add_op() {
        assert_eq!(Op::new("+"), Op::Add);
    }

    #[test]
    fn parse_sub_op() {
        assert_eq!(Op::new("-"), Op::Sub);
    }

    #[test]
    fn parse_mul_op() {
        assert_eq!(Op::new("*"), Op::Mul);
    }

    #[test]
    fn parse_div_op() {
        assert_eq!(Op::new("/"), Op::Div);
    }

    #[test]
    fn parse_one_plus_two() {
        assert_eq!(
            Expr::new("1+2"),
            Expr {
                lhs: Number(1),
                rhs: Number(2),
                op: Op::Add,
            }
        );
    }
}
