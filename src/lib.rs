#![no_std]
extern crate alloc;

pub mod lex;
pub mod utils;
pub mod parse;

pub use utils::Source;
pub use lex::lex;

#[cfg(test)]
mod tests;
