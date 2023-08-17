#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unstable_name_collisions)]

mod dmmr;

mod flip;
mod prototype_rearranging;
mod remap_34;

use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

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
    path: String,
    vars: LinkedHashMap<String, VarVal>,
}

#[derive(Debug, Clone)]
pub struct Prototype {
    id: String,
    atoms: Vec<Atom>,
}

#[derive(Debug, Clone)]
pub struct Row {
    coords: Vec<i32>,
    tiles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Dmm {
    comment: String,
    prototypes: Vec<Prototype>,
    rows: Vec<Row>,
}

fn main() {
    // flip::flip();
    // remap_34::remap();
    prototype_rearranging::remap();
}
