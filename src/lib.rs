#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn leftpad(input: String, len: i32, pat: Option<String>) -> String {
  return pat
    .unwrap_or(String::from(" "))
    .repeat(if len.is_positive() { len as usize } else { 0 })
    + &input;
}
