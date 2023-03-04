use crate::entities::{HTermsGeneratorParam, Term};

pub type TermsPair = (Term, Term);

pub fn generate_h_terms(n: HTermsGeneratorParam) -> TermsPair {
    if n < 1 {
        panic!("n must be greater than 0");
    }

    let mut x_variables = vec![];
    let mut y_variables = vec![];
    let mut f_y_functions = vec![];
    let mut f_x_functions = vec![];

    for i in 1..=n {
        x_variables.push(format!("x{}", i));
        y_variables.push(format!("y{}", i));
        f_y_functions.push(format!("f(y{}, y{})", i - 1, i - 1));
        f_x_functions.push(format!("f(x{}, x{})", i - 1, i - 1));
    }

    let term1 = format!(
        "h({}, {}, {})",
        x_variables.join(", "),
        f_y_functions.join(", "),
        y_variables.last().unwrap()
    );

    let term2 = format!(
        "h({}, {}, {})",
        f_x_functions.join(", "),
        y_variables.join(", "),
        x_variables.last().unwrap()
    );

    (term1, term2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "n must be greater than 0")]
    #[allow(non_snake_case)]
    fn generate_h_terms__n_equals_0__should_panic() {
        generate_h_terms(0);
    }

    #[test]
    #[allow(non_snake_case)]
    fn generate_h_terms__n_equals_1__should_return_correct_terms() {
        let (term1, term2) = generate_h_terms(1);
        assert_eq!(term1, "h(x1, f(y0, y0), y1)");
        assert_eq!(term2, "h(f(x0, x0), y1, x1)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn generate_h_terms__n_equals_2__should_return_correct_terms() {
        let (term1, term2) = generate_h_terms(2);
        assert_eq!(term1, "h(x1, x2, f(y0, y0), f(y1, y1), y2)");
        assert_eq!(term2, "h(f(x0, x0), f(x1, x1), y1, y2, x2)");
    }

    #[test]
    #[allow(non_snake_case)]
    fn generate_h_terms__n_equals_3__should_return_correct_terms() {
        let (term1, term2) = generate_h_terms(3);
        assert_eq!(term1, "h(x1, x2, x3, f(y0, y0), f(y1, y1), f(y2, y2), y3)");
        assert_eq!(term2, "h(f(x0, x0), f(x1, x1), f(x2, x2), y1, y2, y3, x3)");
    }
}
