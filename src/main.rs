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

fn main() {
    // flip::flip();
    // remap_34::remap();
    prototype_rearranging::remap();
}
