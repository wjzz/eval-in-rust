use std::io::{Write};

mod eval;
use eval::EvalCtx;

fn eval(s: &str, eval_ctx: &mut EvalCtx) -> Result<Option<u32>, String> {
    let e = eval::parsing::parse_top(s)?;
    eval_ctx.eval_stm(e)
}


fn main() {
    println!("Press ^C (ctrl+C) to quit.");
    let mut eval_ctx = EvalCtx::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf = buf.trim().to_string();

        match eval(&buf, &mut eval_ctx) {
            Err(err) => println!("> Error! {}", err),
            Ok(Some(n)) => println!("{}", n),
            Ok(None) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(s: &str) -> Result<Option<u32>, String> {
        let e = eval::parsing::parse_top(s)?;
        EvalCtx::new().eval_stm(e)
    }

    #[test]
    fn test_ok_many() {
        let test_cases = [
            ("5", 5),
            ("1+1", 2),
            ("2*3", 6),
            ("1+1+1", 3),
            ("1*2*3*4", 24),
            ("2+2*2", 6),
            ("(2+2)*2", 8),
        ];

        for &(input, output) in test_cases.iter() {
            assert_eq!(Ok(Some(output)), eval(input));
        }
    }

    #[test]
    fn test_assignments_many() {
        let test_cases = ["x = 123", "X = 1 + 1", "abc = 5"];

        for &input in test_cases.iter() {
            assert_eq!(Ok(None), eval(input));
        }
    }

    #[test]
    fn test_failing_many() {
        let test_cases = [
            "",
            "4+",
            "+5",
            "*",
            "+",
            "1+1+",
            "(4",
            "4(",
            "4)",
            "()(",
            "()",
            "1 ++ 2",
            "1 + * 2",
            "1 + ( * 2",
        ];

        for &input in test_cases.iter() {
            assert!(eval(input).is_err());
        }
    }
}
