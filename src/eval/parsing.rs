pub mod token;
use token::Token;

#[derive(Debug, PartialEq, Eq)]
pub enum BinOperator {
    Plus,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Number(u32),
    Var(String),
    BinOp(BinOperator, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Stm {
    EvalExpr(Box<Expr>),
    Assign(String, Box<Expr>),
}

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    fn get_token(&mut self) -> Result<Token, String> {
        match self.tokens.pop() {
            None => Err(String::from("Unexpected end of input")),
            Some(token) => Ok(token),
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        let tk = self.get_token()?;
        if tk == token {
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?} instead", token, tk))
        }
    }

    // Simple backtracking
    fn checkpoint(&mut self) -> Vec<Token> {
        self.tokens.clone()
    }

    fn restore_checkpoint(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens;
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        let token = self.get_token()?;
        match token {
            Token::LParen => {
                let e = self.parse_expr();
                self.expect(Token::RParen)?;
                e
            }
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Var(v) => Ok(Expr::Var(v)),
            _ => Err(format!("Expected {{ or digit, got {:?}", token)),
        }
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_atom()?;
        let token = self.get_token()?;

        if token == Token::Mult {
            let rhs = self.parse_factor()?;
            Ok(Expr::BinOp(BinOperator::Mult, Box::new(lhs), Box::new(rhs)))
        } else {
            self.tokens.push(token);
            Ok(lhs)
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_factor()?;
        let token = self.get_token()?;
        if token == Token::Plus {
            let rhs = self.parse_expr()?;
            Ok(Expr::BinOp(BinOperator::Plus, Box::new(lhs), Box::new(rhs)))
        } else {
            self.tokens.push(token);
            Ok(lhs)
        }
    }

    fn parse_var(&mut self) -> Result<String, String> {
        if let Token::Var(v) = self.get_token()? {
            Ok(v)
        } else {
            Err(String::from("Not a variable!"))
        }
    }

    fn parse_assign(&mut self) -> Result<Stm, String> {
        let v = self.parse_var()?;
        self.expect(Token::Equal)?;
        let e = self.parse_expr()?;
        Ok(Stm::Assign(v, Box::new(e)))
    }

    fn parse_stm(&mut self) -> Result<Stm, String> {
        let checkpoint = self.checkpoint();

        self.parse_assign().or_else(|_err| {
            self.restore_checkpoint(checkpoint);
            Ok(Stm::EvalExpr(Box::new(self.parse_expr()?)))
        })
    }

    fn parse_all(&mut self) -> Result<Stm, String> {
        let e = self.parse_stm();
        self.expect(Token::EOF)?;
        e
    }
}

pub fn parse_top(input: &str) -> Result<Stm, String> {
    let mut tokens = token::tokenize(input)?;
    tokens.reverse();
    Parser::new(tokens).parse_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use BinOperator::*;
    use Expr::*;
    use Stm::*;

    fn parse_top_expr(input: &str) -> Result<Expr, String> {
        let r = parse_top(input)?;
        if let Stm::EvalExpr(e) = r {
            Ok(*e)
        } else {
            Err(String::from("Not an expr"))
        }
    }

    fn lit(n: u32) -> Expr {
        Number(n)
    }

    fn var(s: &str) -> Expr {
        Var(String::from(s))
    }

    fn eq(v: &str, e: Expr) -> Stm {
        Assign(String::from(v), Box::new(e))
    }

    fn plus(e1: Expr, e2: Expr) -> Expr {
        BinOp(Plus, Box::new(e1), Box::new(e2))
    }

    fn mult(e1: Expr, e2: Expr) -> Expr {
        BinOp(Mult, Box::new(e1), Box::new(e2))
    }

    #[test]
    fn test_empty() {
        assert!(parse_top_expr("").is_err());
    }

    #[test]
    fn test_literal() {
        assert_eq!(Ok(lit(10)), parse_top_expr("10"));
    }

    #[test]
    fn test_plus() {
        assert_eq!(Ok(plus(lit(2), lit(3))), parse_top_expr("2+3"));
    }

    #[test]
    fn test_plus_var_left() {
        assert_eq!(Ok(plus(var("x"), lit(3))), parse_top_expr("x+3"));
    }

    #[test]
    fn test_plus_var_right() {
        assert_eq!(Ok(plus(lit(3), var("x"))), parse_top_expr("3+x"));
    }

    #[test]
    fn test_plus_many() {
        assert_eq!(
            Ok(plus(lit(2), plus(lit(3), lit(4)))),
            parse_top_expr("2+3+4")
        );
    }

    #[test]
    fn test_mult() {
        assert_eq!(Ok(mult(lit(2), lit(3))), parse_top_expr("2*3"));
    }

    #[test]
    fn test_mult_many() {
        assert_eq!(
            Ok(mult(lit(2), mult(lit(3), lit(4)))),
            parse_top_expr("2*3*4")
        );
    }

    #[test]
    fn test_mult_plus() {
        assert_eq!(
            Ok(plus(lit(2), mult(lit(3), lit(4)))),
            parse_top_expr("2+3*4")
        );
    }

    #[test]
    fn test_mult_plus2() {
        assert_eq!(
            Ok(plus(mult(lit(2), lit(3)), lit(4))),
            parse_top_expr("2*3+4")
        );
    }

    #[test]
    fn test_parens1() {
        assert_eq!(Ok(lit(2)), parse_top_expr("(2)"));
    }

    #[test]
    fn test_parens2() {
        assert_eq!(Ok(plus(lit(2), lit(3))), parse_top_expr("(2+3)"));
    }

    #[test]
    fn test_parens3() {
        assert_eq!(Ok(plus(lit(2), lit(3))), parse_top_expr("(2)+3"));
    }

    #[test]
    fn test_parens4() {
        assert_eq!(Ok(mult(lit(2), lit(3))), parse_top_expr("(2)*3"));
    }

    #[test]
    fn test_assign1() {
        assert_eq!(Ok(eq("x", lit(5))), parse_top("x = 5"));
    }

    #[test]
    fn test_assign2() {
        assert_eq!(Ok(eq("x", plus(lit(1), lit(5)))), parse_top("x = 1 + 5"));
    }
}
