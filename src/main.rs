#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unstable_name_collisions)]

mod dmmr;

mod all_map_runner;
mod door_fixing;
mod flip;
mod generate;
mod remap_34;
mod wfc;

use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

fn main() {
    // flip::flip();
    // remap_34::remap();
    // door_fixing::remap();
    // all_map_runner::remap();
    // generate::generate_dungen();
    generate::generate_cerberus();
    // wfc::generate_wfc_rs();
    // wfc::generate_wfc_image();
}
