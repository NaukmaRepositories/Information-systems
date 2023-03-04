mod entities;
mod function_parser;
mod h_terms_generator;

use anyhow::{bail, Result};
use entities::{Substitution, Term};
use function_parser::{parse_head_and_tail_of_functional_term, ParserOutput};
use regex::Regex;
use std::{collections::HashMap, time::Instant};

use crate::h_terms_generator::generate_h_terms;

const VARIABLE_RE: &str = r"^[A-Za-z]\d+$";
const CONSTANT_RE: &str = r"^[A-Za-z]$";

/// Checks if either term is constant.
fn constants(term1: &str, term2: &str) -> bool {
    let constant_re = Regex::new(CONSTANT_RE).unwrap();
    constant_re.is_match(term1) || constant_re.is_match(term2)
}

/// Checks if either term is empty.
fn empty(term1: &str, term2: &str) -> bool {
    term1.is_empty() || term2.is_empty()
}

/// Checks if the term is a variable.
fn variable(term: &str) -> bool {
    let variable_re = Regex::new(VARIABLE_RE).unwrap();
    variable_re.is_match(term)
}

/// Checks if the term1 occurs in the term2.
fn occurs(term1: &str, term2: &str) -> bool {
    term2.contains(term1)
}

/// Replaces all occurrences of the keys of the given substitution
/// with the corresponding values in the given term.
fn apply(substituition: &Substitution, term: &str) -> Term {
    let mut result = term.to_owned();

    for (key, value) in substituition.iter() {
        let pattern = format!(r"\b{}\b", key);
        let re = Regex::new(&pattern).unwrap();

        result = re.replace_all(&result, value).to_string();
    }

    result
}

/// Entry point to the unification algorithm.
fn unify(term1: &str, term2: &str) -> Result<Substitution> {
    if term1 == term2 {
        Ok(HashMap::new())
    } else if constants(term1, term2) || empty(term1, term2) {
        bail!(
            "unification failed: different constant or empty terms: term1 — {:?}, term2 — {:?}",
            term1,
            term2
        );
    } else if variable(term1) {
        if occurs(term1, term2) {
            bail!(
                "unification failed: term1 occurs in term2: term1 — {:?}, term2 — {:?}",
                term1,
                term2
            );
        }

        return Ok(HashMap::from([(term1.to_owned(), term2.to_owned())]));
    } else if variable(term2) {
        if occurs(term2, term1) {
            bail!(
                "unification failed: term2 occurs in term1: term2 — {:?}, term1 — {:?}",
                term2,
                term1
            );
        }

        return Ok(HashMap::from([(term2.to_owned(), term1.to_owned())]));
    } else {
        let ParserOutput {
            head: term1_head,
            tail: term1_tail,
        } = parse_head_and_tail_of_functional_term(term1);

        let ParserOutput {
            head: term2_head,
            tail: term2_tail,
        } = parse_head_and_tail_of_functional_term(term2);

        let mut subs1 = unify(&term1_head, &term2_head)?;

        let term1_tail_with_subs = apply(&subs1, &term1_tail);
        let term2_tail_with_subs = apply(&subs1, &term2_tail);

        let subs2 = unify(&term1_tail_with_subs, &term2_tail_with_subs)?;

        subs1.extend(subs2);

        return Ok(subs1);
    }
}

fn main() -> Result<()> {
    let n = 29;
    let (term1, term2) = generate_h_terms(n);

    let start_time = Instant::now();

    let _result = unify(&term1, &term2)?;

    let end_time = Instant::now();
    println!("finished unification with n = {}", n);
    println!("elapsed time: {:?}s", (end_time - start_time).as_secs());

    Ok(())
}

// h(x1, x2, f(y0, y0), f(y1, y1), y2)
// h(f(x0, x0), f(x1, x1), y1, y2, x2)
//
// x1 -> f(x0, x0)
// x2, f(y0, y0), f(y1, y1), y2
// f(f(x0, x0), f(x0, x0)), y1, y2, x2
//
// x2 -> f(f(x0, x0), f(x0, x0))
// f(y0, y0), f(y1, y1), y2
// y1, y2, f(f(x0, x0), f(x0, x0))
//
// y1 -> f(y0, y0)
// f(f(y0, y0), f(y0, y0)), y2
// y2, f(f(x0, x0), f(x0, x0))
//
// y2 -> f(f(y0, y0), f(y0, y0))
// f(f(y0, y0), f(y0, y0))
// f(f(x0, x0), f(x0, x0))
// 
// y0 -> x0
//
