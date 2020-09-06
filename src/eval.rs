use std::collections::HashMap;

pub mod parsing;

use parsing::{BinOperator, Expr, Stm};

pub struct EvalCtx {
    env: HashMap<String, u32>,
}

impl EvalCtx {
    pub fn new() -> Self {
        let env = HashMap::new();
        EvalCtx { env }
    }

    fn eval(&self, e: Expr) -> Result<u32, String> {
        match e {
            Expr::Number(n) => Ok(n),
            Expr::Var(v) => match self.env.get(&v) {
                Some(&val) => Ok(val),
                None => Err(format!("variable not bound: {}", v)),
            },
            Expr::BinOp(op, l, r) => {
                let n1 = self.eval(*l)?;
                let n2 = self.eval(*r)?;
                let r = match op {
                    BinOperator::Plus => n1 + n2,
                    BinOperator::Mult => n1 * n2,
                };
                Ok(r)
            }
        }
    }

    pub fn eval_stm(&mut self, s: Stm) -> Result<Option<u32>, String> {
        match s {
            Stm::EvalExpr(e) => Ok(Some(self.eval(*e)?)),
            Stm::Assign(var, e) => {
                let n = self.eval(*e)?;
                self.env.insert(var, n);
                Ok(None)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn eval(e: Expr) -> Result<u32, String> {
        EvalCtx::new().eval(e)
    }

    #[test]
    fn test_eval_number() {
        assert_eq!(Ok(5), eval(Expr::Number(5)));
    }

    #[test]
    fn test_eval_plus() {
        let l = Box::new(Expr::Number(2));
        let r = Box::new(Expr::Number(3));
        let e = Expr::BinOp(BinOperator::Plus, l, r);

        assert_eq!(Ok(5), eval(e));
    }

    #[test]
    fn test_eval_mult() {
        let l = Box::new(Expr::Number(2));
        let r = Box::new(Expr::Number(3));
        let e = Expr::BinOp(BinOperator::Mult, l, r);

        assert_eq!(Ok(6), eval(e));
    }

    #[test]
    fn test_eval_mult_plus() {
        let l1 = Box::new(Expr::Number(2));
        let l2 = Box::new(Expr::Number(3));
        let e1 = Expr::BinOp(BinOperator::Plus, l1, l2);
        let r = Box::new(Expr::Number(4));
        let e = Expr::BinOp(BinOperator::Mult, Box::new(e1), r);

        assert_eq!(Ok(20), eval(e));
    }
}
