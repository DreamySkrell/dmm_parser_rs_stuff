use std::collections::BTreeMap;

pub use super::*;

pub mod lexer;
pub mod print;
pub mod test;

use grid::Grid;
use lexer::Token;

// ----------------------------------------- types:

#[derive(Debug, Clone)]
pub enum VarVal {
    String(String),
    Path(String),
    Null,
    Int(f64),
    List(Vec<i32>),
    ListString(Vec<String>),
    ListStringAssoc(Vec<(String, String)>),
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub path: String,
    pub vars: LinkedHashMap<String, VarVal>,
}

#[derive(Debug, Clone, Default)]
pub struct Prototype {
    pub id: String,
    pub atoms: Vec<Atom>,
}

#[derive(Debug, Clone, Default)]
pub struct Row {
    pub coords: Vec<i32>,
    pub tiles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Dmm {
    pub comment: String,
    pub prototypes: Vec<Prototype>,
    pub rows: Vec<Row>,
}

pub struct Umm {
    pub comment: String,
    pub grid: Grid<Prototype>,
}

// ----------------------------------------- ^types

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

pub fn unpack(dmm: &Dmm) -> Umm {
    let mut umm = Umm {
        comment: dmm.comment.clone(),
        grid: Grid::new(dmm.rows.len(), dmm.rows.first().unwrap().tiles.len()),
    };

    let mut prototypes = HashMap::<String, Prototype>::new();

    for prototype in &dmm.prototypes {
        prototypes.insert(prototype.id.clone(), prototype.clone());
    }

    for (x, row) in dmm.rows.iter().enumerate() {
        for (y, tile) in row.tiles.iter().enumerate() {
            *umm.grid.get_mut(x, y).unwrap() = prototypes.get(tile).unwrap().clone();
        }
    }

    umm
}

pub fn pack(umm: &Umm) -> Dmm {
    let mut dmm = Dmm {
        comment: umm.comment.clone(),
        prototypes: Vec::new(),
        rows: Vec::new(),
    };

    let mut prototypes = BTreeMap::<String, Prototype>::new();

    let rows = umm.grid.rows();
    let cols = umm.grid.cols();

    dmm.rows.resize(rows, Row::default());
    for (row_i, row) in dmm.rows.iter_mut().enumerate() {
        row.tiles.resize(cols, "".to_string());
        row.coords = vec![(row_i + 1).try_into().unwrap(), 1, 1];
        for (col_i, col) in row.tiles.iter_mut().enumerate() {
            let prototype_at_tile = umm.grid.get(row_i, col_i).unwrap();
            if !prototypes.contains_key(&prototype_at_tile.id) {
                prototypes.insert(prototype_at_tile.id.clone(), prototype_at_tile.clone());
                *col = prototype_at_tile.id.clone();
            } else {
                *col = prototype_at_tile.id.clone();
            }
        }
    }

    for (_id, prototype) in prototypes
        .iter() //
        .sorted_by(|(a_i, _a_p), (b_i, _b_p)| {
            //
            let a_char = a_i.chars().next().unwrap();
            let b_char = b_i.chars().next().unwrap();
            if a_char.is_uppercase() && b_char.is_lowercase() {
                std::cmp::Ordering::Greater
            } else if a_char.is_lowercase() && b_char.is_uppercase() {
                std::cmp::Ordering::Less
            } else {
                Ord::cmp(a_i, b_i)
            }
        })
    {
        dmm.prototypes.push(prototype.clone());
    }

    dmm
}
