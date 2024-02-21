#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

use grid::Grid;
use simdnoise::NoiseBuilder;

pub fn apply() {
    let map_dir = "D:/Git/Aurora.3/maps".to_string();
    let map_name = "autopipe";
    let origin_path: std::path::PathBuf = format!("{map_dir}/{map_name}_in.dmm").into();
    let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_pp.dmm").into();
    let result_path: std::path::PathBuf = format!("{map_dir}/{map_name}_ou.dmm").into();

    let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
    let parsed = parse(&origin_map_str);

    // let parsed_str = print(&parsed);
    // std::fs::write(parsed_path, parsed_str.clone()).unwrap();

    let mut umm = unpack(&parsed);

    let rows = umm.grid.rows();
    let cols = umm.grid.cols();

    let autopipe_config = [(
        "/obj/machinery/atmospherics/pipe/simple/hidden/supply",
        "/obj/machinery/atmospherics/pipe/manifold/hidden/supply",
        "/obj/machinery/atmospherics/pipe/manifold4w/hidden/supply",
        "/obj/machinery/atmospherics/unary/vent_pump/on",
    )];

    for row in 0..rows {
        for col in 0..cols {
            if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
                continue;
            }

            let atoms_n = umm.grid.get(row, col + 1).unwrap().atoms.clone();
            let atoms_s = umm.grid.get(row, col - 1).unwrap().atoms.clone();
            let atoms_e = umm.grid.get(row + 1, col).unwrap().atoms.clone();
            let atoms_w = umm.grid.get(row - 1, col).unwrap().atoms.clone();

            let connects_to_n = true;
            let connects_to_s = true;
            let connects_to_e = true;
            let connects_to_w = true;

            let prototypes = umm.grid.get_mut(row, col).unwrap();
            let atoms = &mut prototypes.atoms;

            for (pipe, mani3w, mani4w, vent) in autopipe_config {
                for atom in prototypes.atoms.iter_mut() {
                    if atom.path == mani4w {
                        if connects_to_n && connects_to_s && connects_to_e && connects_to_w {
                            continue;
                        }
                        if connects_to_n && connects_to_s {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(2f64));
                        }
                        if connects_to_e && connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(4f64));
                        }
                    }
                }
            }
        }
    }

    for x in umm.grid.iter_mut() {
        while x.id.len() < 3 {
            x.id = format!("a{}", x.id);
        }
    }
    let repacked = pack(&umm);

    let result_str = print(&repacked);

    std::fs::write(result_path, result_str).unwrap();
}
