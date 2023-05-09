#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn leftpad(original: String, len: u32, pat: Option<String>) -> String {
  return pat.unwrap_or(String::from(" ")).repeat(len as usize) + &original;
}
