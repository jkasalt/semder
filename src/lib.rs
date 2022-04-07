use once_cell::sync::Lazy;
use regex::Regex;
use std::rc::Rc;
use thiserror::Error;

pub mod parsing;

static RE_PAREN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\((.*)\)").expect("Creating parenthesis regex"));

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Cannot parse expression")]
    ParseErr(String),
}

#[derive(Debug, PartialEq)]
pub enum Ope {
    Add(Rc<Ope>, Rc<Ope>),
    Mul(Rc<Ope>, Rc<Ope>),
    Pow(f64),
    Cst(Cst),
}

#[derive(Debug, PartialEq)]
pub enum Cst {
    Num(f64),
    Sem(char),
}

impl Ope {
    pub fn grad(&self) -> Self {
        match self {
            Ope::Add(f1, f2) => Ope::Add(Rc::new(f1.grad()), Rc::new(f2.grad())),
            Ope::Mul(f1, f2) => Ope::Add(
                Rc::new(Ope::Mul(Rc::new(f1.grad()), f2.clone())),
                Rc::new(Ope::Mul(f1.clone(), Rc::new(f2.grad()))),
            ),
            Ope::Pow(n) => Ope::Mul(Rc::new(Ope::Cst(Cst::Num(*n))), Rc::new(Ope::Pow(*n - 1.0))),
            Ope::Cst(_) => Ope::Cst(Cst::Num(0.0)),
        }
    }
}

impl std::str::FromStr for Ope {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod grad {
        use super::*;

        #[test]
        fn constant() {
            assert_eq!(Ope::Cst(Cst::Num(32.0)).grad(), Ope::Cst(Cst::Num(0.0)));
        }

        macro_rules! add {
            ($a:expr, $b:expr) => {
                Ope::Add(
                    Rc::new(Ope::Cst(Cst::Num($a))),
                    Rc::new(Ope::Cst(Cst::Num($b))),
                )
            };
        }

        #[test]
        fn add_simple() {
            let add = add!(2.0, 3.0);
            assert_eq!(add.grad(), add!(0.0, 0.0));
        }
    }

    mod from_string {
        use super::*;
        use std::rc::Rc;
        use std::str::FromStr;

        #[test]
        fn simple() {
            assert_eq!(
                Ope::from_str("x + 1"),
                Ok(Ope::Add(
                    Rc::new(Ope::Pow(1.0)),
                    Rc::new(Ope::Cst(Cst::Num(1.0)))
                ))
            );
        }

        #[test]
        fn squared() {
            assert_eq!(
                Ope::from_str("x^2 + 1"),
                Ok(Ope::Add(
                    Rc::new(Ope::Pow(2.0)),
                    Rc::new(Ope::Cst(Cst::Num(1.0)))
                ))
            );
        }

        #[test]
        #[ignore]
        fn one_parenthesis() {
            assert_eq!(
                Ope::from_str("(x + 1) + a").unwrap(),
                Ope::Add(
                    Rc::new(Ope::Add(
                        Rc::new(Ope::Pow(1.0)),
                        Rc::new(Ope::Cst(Cst::Num(1.0)))
                    )),
                    Rc::new(Ope::Cst(Cst::Sem('a')))
                )
            );
        }
    }
}
