pub mod compiler;
pub mod lexer;
pub mod tokens;

mod arena;
pub mod ast;
pub mod datatypes;
mod debug;
mod errors;
pub mod generator;
pub mod parse;
mod symbols;

pub struct Deferable<F: FnOnce()>(Option<F>);
impl<F: FnOnce()> Deferable<F> {
    pub fn new(f: F) -> Self {
        Self(Some(f))
    }
}
