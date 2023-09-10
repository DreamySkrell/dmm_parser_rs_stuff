use std::collections::{BTreeMap, BTreeSet};

pub use super::*;

pub mod lexer;
pub mod print;
pub mod test;

use grid::Grid;
use itertools::Itertools;
use lexer::Token;
use linked_hash_map::LinkedHashMap;

// ----------------------------------------- types:

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum VarVal {
    String(String),
    Icon(String),
    Path(String),
    Null,
    Int(f64),
    List(Vec<i32>),
    ListString(Vec<String>),
    ListStringAssoc(Vec<(String, String)>),
    ListPath(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Atom {
    pub path: String,
    pub vars: LinkedHashMap<String, VarVal>,
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
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

    let mut prototypes = std::collections::HashMap::<String, Prototype>::new();

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

    // find all taken prototype ids
    let mut prototype_ids_taken = BTreeSet::<String>::new();
    let mut prototype_ids_free = BTreeSet::<String>::new();
    {
        for tile in umm.grid.iter() {
            prototype_ids_taken.insert(tile.id.clone());
        }
        let id_chars: Vec<char> = ('a'..'z')
            // .chain('A'..'Z') /*.chain('0'..'9')*/
            .collect();
        match umm.grid.iter().next().unwrap().id.chars().count() {
            1 => {
                for a in id_chars.iter() {
                    let id: String = [*a].iter().collect();
                    if !prototype_ids_taken.contains(&id) {
                        prototype_ids_free.insert(id);
                    }
                }
            }
            2 => {
                for (a, b) in id_chars.iter().cartesian_product(id_chars.iter()) {
                    let id: String = [*a, *b].iter().collect();
                    if !prototype_ids_taken.contains(&id) {
                        prototype_ids_free.insert(id);
                    }
                }
            }
            3 => {
                for ((a, b), c) in id_chars
                    .iter()
                    .cartesian_product(id_chars.iter())
                    .cartesian_product(id_chars.iter())
                {
                    let id: String = [*a, *b, *c].iter().collect();
                    if !prototype_ids_taken.contains(&id) {
                        prototype_ids_free.insert(id);
                    }
                }
            }
            _ => panic!(),
        }
    }

    //
    let mut prototypes = BTreeMap::<String, Prototype>::new();

    let rows = umm.grid.rows();
    let cols = umm.grid.cols();

    // collect tiles into dmm and write prototypes to map
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
                let prototype_in_map = prototypes.get(&prototype_at_tile.id).unwrap().clone();
                if prototype_in_map == *prototype_at_tile {
                    *col = prototype_at_tile.id.clone();
                } else {
                    let new_id = prototype_ids_free.iter().next().unwrap().clone();
                    prototype_ids_free.remove(&new_id);
                    let mut new_prototype = prototype_at_tile.clone();
                    new_prototype.id = new_id.clone();
                    prototypes.insert(new_prototype.id.clone(), new_prototype.clone());
                    *col = new_id.clone();
                }
            }
        }
    }

    // collect prototypes from map into dmm
    for (_id, prototype) in prototypes
        .iter() //
        .sorted_by(|(a_i, _a_p), (b_i, _b_p)| {
            let switch_case = |s: &String| {
                s.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            c.to_ascii_lowercase()
                        } else {
                            c.to_ascii_uppercase()
                        }
                    })
                    .collect::<String>()
            };
            let a_i_switched_case = switch_case(a_i);
            let b_i_switched_case = switch_case(b_i);
            Ord::cmp(&a_i_switched_case, &b_i_switched_case)
            //
        })
    {
        dmm.prototypes.push(prototype.clone());
    }

    dmm
}
