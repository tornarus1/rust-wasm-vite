use wasm_bindgen::prelude::*;

type Result<T> = std::result::Result<T, JsError>;

/// Divides `lhs` by `rhs`.
///
/// ## Errors
/// Propagates errors if `rhs` is 0.
#[wasm_bindgen]
pub fn div(lhs: i32, rhs: i32) -> Result<i32> {
    // `?` is here to simply coerce `CalcError` into `JsError` via `Display`
    Ok(calculator::div(lhs, rhs)?)
}
