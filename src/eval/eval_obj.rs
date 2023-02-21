use std::{
    fmt::Debug,
    ops::{Add, Div, Mul, Rem, Sub},
};

use super::error::EvalError;

#[derive(PartialEq, Clone, Copy)]
pub(crate) enum EvalObj {
    NONE,
    Integer(i32),
    Number(f64),
}

impl Debug for EvalObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalObj::NONE => write!(f, "nothing"),
            EvalObj::Integer(int) => write!(f, "{}", int),
            EvalObj::Number(float) => write!(f, "{}", float),
        }
    }
}

impl Add for EvalObj {
    type Output = Result<EvalObj, EvalError>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalObj::NONE, EvalObj::NONE) => Ok(EvalObj::NONE),
            (EvalObj::Integer(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Integer(lhs + rhs)),
            (EvalObj::Number(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Number(lhs + rhs as f64)),
            (EvalObj::Integer(lhs), EvalObj::Number(rhs)) => Ok(EvalObj::Number(lhs as f64 + rhs)),

            _ => Err(EvalError::type_error(
                "the type of lhs is not same to rhs".to_string(),
            )),
        }
    }
}

impl Sub for EvalObj {
    type Output = Result<EvalObj, EvalError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalObj::NONE, EvalObj::NONE) => Ok(EvalObj::NONE),
            (EvalObj::Integer(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Integer(lhs - rhs)),
            (EvalObj::Number(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Number(lhs - rhs as f64)),
            (EvalObj::Integer(lhs), EvalObj::Number(rhs)) => Ok(EvalObj::Number(lhs as f64 - rhs)),

            _ => Err(EvalError::type_error(
                "the type of lhs is not same to rhs".to_string(),
            )),
        }
    }
}

impl Mul for EvalObj {
    type Output = Result<EvalObj, EvalError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalObj::NONE, EvalObj::NONE) => Ok(EvalObj::NONE),
            (EvalObj::Integer(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Integer(lhs * rhs)),
            (EvalObj::Number(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Number(lhs * rhs as f64)),
            (EvalObj::Integer(lhs), EvalObj::Number(rhs)) => Ok(EvalObj::Number(lhs as f64 * rhs)),

            _ => Err(EvalError::type_error(
                "the type of lhs is not same to rhs".to_string(),
            )),
        }
    }
}

impl Div for EvalObj {
    type Output = Result<EvalObj, EvalError>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalObj::NONE, EvalObj::NONE) => Ok(EvalObj::NONE),
            (_, EvalObj::Integer(0)) => Err(EvalError::divide_zero_error()),
            (_, EvalObj::Number(n)) if n == 0.0 => Err(EvalError::divide_zero_error()),

            (EvalObj::Integer(lhs), EvalObj::Integer(rhs)) => {
                let res1 = lhs as f64 / rhs as f64;
                let res2 = lhs / rhs;
                if res1 == res2 as f64 {
                    Ok(EvalObj::Integer(res2))
                } else {
                    Ok(EvalObj::Number(res1))
                }
            }

            (EvalObj::Number(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Number(lhs / rhs as f64)),
            (EvalObj::Integer(lhs), EvalObj::Number(rhs)) => Ok(EvalObj::Number(lhs as f64 / rhs)),

            _ => Err(EvalError::type_error(
                "the type of lhs is not same to rhs".to_string(),
            )),
        }
    }
}

impl Rem for EvalObj {
    type Output = Result<EvalObj, EvalError>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EvalObj::NONE, EvalObj::NONE) => Ok(EvalObj::NONE),
            (_, EvalObj::Integer(0)) => Err(EvalError::divide_zero_error()),
            (EvalObj::Integer(lhs), EvalObj::Integer(rhs)) => Ok(EvalObj::Integer(lhs % rhs)),

            _ => Err(EvalError::type_error(
                "the type of lhs is not same to rhs".to_string(),
            )),
        }
    }
}
