#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unstable_name_collisions)]

mod flip;
mod lexer;
mod test;

use itertools::Itertools;
use lalrpop_util::lalrpop_mod;
use lexer::*;
use std::collections::HashMap;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

pub enum VarVal {
    String(String),
    Path(String),
    Null,
    Int(f64),
    List(Vec<i32>),
    ListString(Vec<String>),
}

pub struct Var {
    name: String,
    val: VarVal,
}

pub struct Atom {
    path: String,
    vars: Vec<Var>,
}

pub struct Prototype {
    id: String,
    atoms: Vec<Atom>,
}

pub struct Row {
    coords: Vec<i32>,
    tiles: Vec<String>,
}

pub struct Dmm {
    comment: String,
    prototypes: Vec<Prototype>,
    rows: Vec<Row>,
}

fn parse(dmm: &str) -> Dmm {
    let tokens: Vec<(usize, Token, usize)> =
        lexe(dmm).iter().map(|(n, t)| (*n, t.clone(), 0)).collect();

    // for (i, t, _) in &tokens {
    //     println!("{}: {:?}", i, t);
    // }

    parser::DmmParser::new().parse(tokens).unwrap()
}

fn newline() -> &'static str {
    "\n"
}

fn tabchar() -> &'static str {
    "\t"
}

fn print(dmm: &Dmm) -> String {
    let mut s = String::new();

    // comment
    s.push_str(&format!("//{}{}", dmm.comment, newline()));

    // prototypes
    for proto in &dmm.prototypes {
        s.push_str(&format!("\"{}\" = (", proto.id));
        for (i, atom) in proto.atoms.iter().enumerate() {
            s.push_str(&format!("{}", newline()));
            s.push_str(&format!("{}", atom.path));

            if !atom.vars.is_empty() {
                s.push_str(&format!("{{"));
                for (ii, var) in atom.vars.iter().enumerate() {
                    s.push_str(&format!("{}", newline()));
                    match &var.val {
                        VarVal::String(ss) => {
                            s.push_str(&format!("{}{} = \"{}\"", tabchar(), var.name, ss));
                        }
                        VarVal::Path(ss) => {
                            s.push_str(&format!("{}{} = '{}'", tabchar(), var.name, ss));
                        }
                        VarVal::Null => {
                            s.push_str(&format!("{}{} = {}", tabchar(), var.name, "null"));
                        }
                        VarVal::Int(i) => {
                            s.push_str(&format!("{}{} = {}", tabchar(), var.name, i));
                        }
                        VarVal::List(l) => {
                            s.push_str(&format!(
                                "{}{} = list({})",
                                tabchar(),
                                var.name,
                                l.iter()
                                    .map(|i| i.to_string())
                                    .intersperse(",".into())
                                    .collect::<String>()
                            ));
                        }
                        VarVal::ListString(l) => {
                            s.push_str(&format!(
                                "{}{} = list(\"{}\")",
                                tabchar(),
                                var.name,
                                l.iter()
                                    .map(|i| i.to_string())
                                    .intersperse("\",\"".into())
                                    .collect::<String>()
                            ));
                        }
                    }

                    if ii < atom.vars.len() - 1 {
                        s.push_str(&format!(";"));
                    }
                }
                s.push_str(&format!("{}{}}}", newline(), tabchar()));
            }

            if i < proto.atoms.len() - 1 {
                s.push_str(&format!(","));
            }
        }
        s.push_str(&format!("){}", newline()));
    }

    // break
    s.push_str(&format!("{}", newline()));

    // rows
    for row in &dmm.rows {
        s.push_str(&format!(
            "({},{},{}) = {{\"{}",
            row.coords[0],
            row.coords[1],
            row.coords[2],
            newline(),
        ));
        for tile in &row.tiles {
            s.push_str(&format!("{}{}", tile, newline()));
        }
        s.push_str(&format!("\"}}{}", newline()));
    }

    // done
    s
}

fn main() {
    //let dmm = std::fs::read_to_string("data/x.dmm").unwrap();
    //let dmm = parser::DmmParser::new().parse(&dmm).unwrap();
}
