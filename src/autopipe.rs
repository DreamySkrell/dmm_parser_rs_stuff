#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

use grid::Grid;
use itertools::Itertools;
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

            for (pipe, mani3w, mani4w, vent) in autopipe_config {
                let atoms_n = umm.grid.get(row, col - 1).unwrap().atoms.clone();
                let atoms_s = umm.grid.get(row, col + 1).unwrap().atoms.clone();
                let atoms_e = umm.grid.get(row + 1, col).unwrap().atoms.clone();
                let atoms_w = umm.grid.get(row - 1, col).unwrap().atoms.clone();

                let get_gipe_from_atoms = |atoms: &Vec<Atom>| {
                    atoms
                        .iter()
                        .find_or_first(|a| a.path == pipe)
                        .map(|a| {
                            (
                                a.path.clone(),
                                a.vars
                                    .get("dir")
                                    .map(|dir| {
                                        if let VarVal::Int(dir) = dir {
                                            *dir as i32
                                        } else {
                                            4
                                        }
                                    })
                                    .unwrap_or(2),
                            )
                        })
                        .unwrap()
                };

                let any_eq = |l: &[i32], e: i32| l.iter().any(|t| *t == e);

                let (n_path, n_dir) = get_gipe_from_atoms(&atoms_n);
                let (s_path, s_dir) = get_gipe_from_atoms(&atoms_s);
                let (e_path, e_dir) = get_gipe_from_atoms(&atoms_e);
                let (w_path, w_dir) = get_gipe_from_atoms(&atoms_w);

                let connects_to_n: bool = {
                    if n_path == mani4w {
                        true
                    } else if n_path == pipe && any_eq(&[1, 2, 10, 6], n_dir) {
                        true
                    } else if n_path == mani3w && any_eq(&[1, 4, 8], n_dir) {
                        true
                    } else if n_path == vent && any_eq(&[2], n_dir) {
                        true
                    } else {
                        false
                    }
                };
                let connects_to_s = {
                    if s_path == mani4w {
                        true
                    } else if s_path == pipe && any_eq(&[1, 2, 5, 9], s_dir) {
                        true
                    } else if s_path == mani3w && any_eq(&[2, 4, 8], s_dir) {
                        true
                    } else if s_path == vent && any_eq(&[1], s_dir) {
                        true
                    } else {
                        false
                    }
                };
                let connects_to_e = {
                    if e_path == mani4w {
                        true
                    } else if e_path == pipe && any_eq(&[4, 8, 10, 9], e_dir) {
                        true
                    } else if e_path == mani3w && any_eq(&[1, 2, 4], e_dir) {
                        true
                    } else if e_path == vent && any_eq(&[8], e_dir) {
                        true
                    } else {
                        false
                    }
                };
                let connects_to_w = {
                    if w_path == mani4w {
                        true
                    } else if w_path == pipe && any_eq(&[4, 8, 5, 6], w_dir) {
                        true
                    } else if w_path == mani3w && any_eq(&[1, 2, 8], w_dir) {
                        true
                    } else if w_path == vent && any_eq(&[4], w_dir) {
                        true
                    } else {
                        false
                    }
                };

                let prototypes = umm.grid.get_mut(row, col).unwrap();
                let atoms = &mut prototypes.atoms;

                for atom in prototypes.atoms.iter_mut() {
                    if atom.path == mani4w {
                        // mani4w
                        if connects_to_n && connects_to_s && connects_to_e && connects_to_w {
                            continue;
                        }

                        // pipe straight
                        if connects_to_n && connects_to_s && !connects_to_e && !connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(2f64));
                            continue;
                        }
                        if !connects_to_n && !connects_to_s && connects_to_e && connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(4f64));
                            continue;
                        }

                        // pipe end
                        if (connects_to_n ^ connects_to_s) && !connects_to_e && !connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(2f64));
                            continue;
                        }
                        if !connects_to_n && !connects_to_s && (connects_to_e ^ connects_to_w) {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(4f64));
                            continue;
                        }

                        // pipe turn
                        if connects_to_n && !connects_to_s && connects_to_e && !connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(5f64));
                            continue;
                        }
                        if !connects_to_n && connects_to_s && !connects_to_e && connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars
                                .insert("dir".to_string(), dmmr::VarVal::Int(10f64));
                            continue;
                        }
                        if connects_to_n && !connects_to_s && !connects_to_e && connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(9f64));
                            continue;
                        }
                        if !connects_to_n && connects_to_s && connects_to_e && !connects_to_w {
                            atom.path = pipe.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(6f64));
                            continue;
                        }

                        // mani3w
                        if !connects_to_n && connects_to_s && connects_to_e && connects_to_w {
                            atom.path = mani3w.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(1f64));
                            continue;
                        }
                        if connects_to_n && !connects_to_s && connects_to_e && connects_to_w {
                            atom.path = mani3w.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(2f64));
                            continue;
                        }
                        if connects_to_n && connects_to_s && !connects_to_e && connects_to_w {
                            atom.path = mani3w.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(4f64));
                            continue;
                        }
                        if connects_to_n && connects_to_s && connects_to_e && !connects_to_w {
                            atom.path = mani3w.to_string();
                            atom.vars.insert("dir".to_string(), dmmr::VarVal::Int(8f64));
                            continue;
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
