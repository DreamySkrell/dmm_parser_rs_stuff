pub use super::*;

pub mod lexer;
pub mod print;
pub mod test;

use lexer::Token;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub parser, "/dmmr/parser.rs"); // synthesized by LALRPOP

pub fn parse(dmm: &str) -> Dmm {
    use crate::dmmr::lexer::Token;
    let tokens: Vec<(usize, Token, usize)> = lexer::lexe(dmm)
        .iter()
        .map(|(n, t)| (*n, t.clone(), 0))
        .collect();
    parser::DmmParser::new().parse(tokens).unwrap()
}

pub fn print(dmm: &Dmm) -> String {
    print::print(dmm)
}
