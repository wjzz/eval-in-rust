#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    Number(u32),
    Var(String),
    LParen,
    RParen,
    Plus,
    Mult,
    Equal,
    EOF,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    use Token::*;

    // add a sentinel at the end
    let input = format!("{} ", input);

    let mut tokens = vec![];

    let mut is_number = false;
    let mut digit_buffer = 0;

    let mut is_var = false;
    let mut var_buffer = String::new();

    for ch in input.chars() {
        if ch.is_ascii_digit() {
            digit_buffer *= 10;
            digit_buffer += ch.to_digit(10).unwrap();
            is_number = true;
        } else if ch.is_ascii_alphabetic() {
            var_buffer.push(ch);
            is_var = true;
        } else {
            if is_number {
                tokens.push(Number(digit_buffer));
                digit_buffer = 0;
                is_number = false;
            } else if is_var {
                tokens.push(Var(var_buffer));
                var_buffer = String::new();
                is_var = false;
            }
            match ch {
                ' ' | '\n' => continue,
                '=' => tokens.push(Equal),
                '(' => tokens.push(LParen),
                ')' => tokens.push(RParen),
                '+' => tokens.push(Plus),
                '*' => tokens.push(Mult),
                _ => return Err(format!("Unexpected character: {}", ch)),
            }
        }
    }

    tokens.push(EOF);
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token::*};
    #[test]
    fn test_empty() {
        assert_eq!(tokenize("").unwrap(), vec![EOF])
    }

    #[test]
    fn test_number() {
        assert_eq!(tokenize("123").unwrap(), vec![Number(123), EOF])
    }

    #[test]
    fn test_number_spaces() {
        assert_eq!(tokenize("   123   ").unwrap(), vec![Number(123), EOF])
    }

    #[test]
    fn test_numbers() {
        assert_eq!(
            tokenize("1 2 3").unwrap(),
            vec![Number(1), Number(2), Number(3), EOF]
        )
    }

    #[test]
    fn test_vars() {
        assert_eq!(
            tokenize("a x sum").unwrap(),
            vec![
                Var(String::from("a")),
                Var(String::from("x")),
                Var(String::from("sum")),
                EOF
            ]
        )
    }

    #[test]
    fn test_numbers2() {
        assert_eq!(
            tokenize("1 + 2*3").unwrap(),
            vec![Number(1), Plus, Number(2), Mult, Number(3), EOF]
        )
    }

    #[test]
    fn test_mix() {
        assert_eq!(
            tokenize("x + 5").unwrap(),
            vec![Var(String::from("x")), Plus, Number(5), EOF]
        )
    }

    #[test]
    fn test_eq() {
        assert_eq!(
            tokenize("x = 5").unwrap(),
            vec![Var(String::from("x")), Equal, Number(5), EOF]
        )
    }
}
