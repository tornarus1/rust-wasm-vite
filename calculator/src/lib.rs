use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CalcError {
    #[error("cannot divide by 0")]
    DivByZero,

    #[error("unknown error")]
    Unknown,
}

/// Divides `lhs` by `rhs`.
///
/// ## Errors
/// Returns an error if `rhs` is 0.
pub fn div(lhs: i32, rhs: i32) -> Result<i32, CalcError> {
    if rhs == 0 {
        return Err(CalcError::DivByZero);
    }

    Ok(lhs / rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_works() {
        assert_eq!(div(2, 2).unwrap(), 1);

        assert!(div(2, 0).is_err_and(|e| e == CalcError::DivByZero));
    }
}
