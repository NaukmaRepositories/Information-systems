use crate::entities::Term;

pub struct ParserOutput {
    pub head: Term,
    pub tail: Term,
}

// h(f(x0, g(x1, x1)), f(x1, x1), y1, y2, x2)
pub fn parse_head_and_tail_of_functional_term(func: &str) -> ParserOutput {
    let arguments_str = &func[2..func.len() - 1];
    let mut parentheses_counter = 0;

    let mut arguments = vec![];
    let mut current_argument = String::new();

    for symbol in arguments_str.chars() {
        let mut is_separator = false;

        match symbol {
            '(' => {
                parentheses_counter += 1;
            }
            ')' => {
                parentheses_counter -= 1;
            }
            ',' => {
                if parentheses_counter == 0 {
                    is_separator = true;
                    arguments.push(current_argument.trim().to_owned());
                    current_argument.clear();
                }
            }
            _ => ()
        }

        if !is_separator {
            current_argument.push(symbol);
        }
    }

    if !current_argument.is_empty() {
        arguments.push(current_argument.trim().to_owned());
    }

    match arguments.len() {
        0 => ParserOutput {
            head: String::new(),
            tail: String::from("d()")
        },
        1 => ParserOutput {
            head: arguments[0].clone(),
            tail: String::from("d()")
        },
        _ => ParserOutput {
            head: arguments[0].clone(),
            tail: format!("d({})", arguments[1..].join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn parse_head_and_tail_of_functional_term__complex_function__should_return_correct_head_and_tail() {
        let func = "h(f(x0, g(x1, x1)), f(x1, x1), y1, y2, x2)";
        let expected_head = "f(x0, g(x1, x1))";
        let expected_tail = "d(f(x1, x1), y1, y2, x2)";

        let ParserOutput { head, tail } = parse_head_and_tail_of_functional_term(func);

        assert_eq!(head, expected_head);
        assert_eq!(tail, expected_tail);
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_head_and_tail_of_functional_term__empty_function__should_return_correct_head_and_tail() {
        let func = "h()";
        let expected_head = "";
        let expected_tail = "d()";

        let ParserOutput { head, tail } = parse_head_and_tail_of_functional_term(func);

        assert_eq!(head, expected_head);
        assert_eq!(tail, expected_tail);
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_head_and_tail_of_functional_term__function_with_one_argument__should_return_correct_head_and_tail() {
        let func = "h(f(x0, g(x1, x1)))";
        let expected_head = "f(x0, g(x1, x1))";
        let expected_tail = "d()";

        let ParserOutput { head, tail } = parse_head_and_tail_of_functional_term(func);

        assert_eq!(head, expected_head);
        assert_eq!(tail, expected_tail);
    }
}
