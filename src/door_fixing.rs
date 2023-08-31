#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

pub fn remap() {
    let map_dir = "D:/Git/Aurora.3.kyres1/maps/sccv_horizon".to_string();
    let map_name = "sccv_horizon-3_deck_3";
    let origin_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();
    let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_p.dmm").into();
    let result_path: std::path::PathBuf = format!("{map_dir}/{map_name}_r.dmm").into();

    let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
    let parsed = parse(&origin_map_str);

    let parsed_str = print(&parsed);
    std::fs::write(parsed_path, parsed_str.clone()).unwrap();
    // for (i, diff) in diff::lines(&origin_map_str, &parsed_str).iter().enumerate() {
    //     match diff {
    //         diff::Result::Left(l) => println!("{} diff - : {}", i, l),
    //         diff::Result::Both(l, r) => {
    //             assert_eq!(l, r);
    //         }
    //         diff::Result::Right(r) => println!("{} diff + : {}", i, r),
    //     }
    // }
    assert!(origin_map_str == parsed_str);

    let mut umm = unpack(&parsed);

    let rows = umm.grid.rows();
    let cols = umm.grid.cols();
    for row in 0..rows {
        for col in 0..cols {
            if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
                continue;
            }

            // /turf/simulated/floor
            // /turf/simulated/wall
            // /obj/machinery/door/airlock

            let prototypes_cloned = umm.grid.get(row, col).unwrap().clone();

            let is_turf_floor = |a: &Atom| a.path.starts_with("/turf/simulated/floor");
            let north_turf_floor = umm
                .grid
                .get(row + 1, col)
                .unwrap()
                .atoms
                .iter()
                .any(is_turf_floor);
            let south_turf_floor = umm
                .grid
                .get(row - 1, col)
                .unwrap()
                .atoms
                .iter()
                .any(is_turf_floor);
            let east_turf_floor = umm
                .grid
                .get(row, col + 1)
                .unwrap()
                .atoms
                .iter()
                .any(is_turf_floor);
            let west_turf_floor = umm
                .grid
                .get(row, col - 1)
                .unwrap()
                .atoms
                .iter()
                .any(is_turf_floor);

            let prototypes = umm.grid.get_mut(row, col).unwrap();

            for atom in prototypes.atoms.iter_mut() {
                if atom.path.starts_with("/obj/machinery/door/airlock") {
                    if north_turf_floor && south_turf_floor {
                        atom.vars
                            .insert("dir".to_string(), dmmr::VarVal::Int(4.0f64));
                    } else if east_turf_floor && west_turf_floor {
                        atom.vars
                            .insert("dir".to_string(), dmmr::VarVal::Int(1.0f64));
                    }
                }
            }
        }
    }

    let repacked = pack(&umm);

    let result_str = print(&repacked);

    std::fs::write(result_path, result_str).unwrap();
}
