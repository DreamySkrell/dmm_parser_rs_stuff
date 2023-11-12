#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

use grid::Grid;
use simdnoise::NoiseBuilder;

pub fn apply() {
    let map_dir = "D:/Git/Aurora.3/maps/event/live_cooking_demonstration".to_string();
    let map_name = "live_cooking_demonstration";
    let origin_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();
    let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();
    let result_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();

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
    // assert!(origin_map_str == parsed_str);

    let mut umm = unpack(&parsed);

    let rows = umm.grid.rows();
    let cols = umm.grid.cols();

    let noise1: Grid<_> = Grid::from_vec(
        NoiseBuilder::fbm_2d(255, 255).generate_scaled(0.0, 6.0),
        cols,
    );
    let noise: Grid<_> = Grid::from_vec(
        NoiseBuilder::fbm_2d(255, 255).generate_scaled(0.0, 6.0),
        cols,
    );

    for row in 0..rows {
        for col in 0..cols {
            if col == 0 || col == cols - 1 || row == 0 || row == rows - 1 {
                continue;
            }

            let prototypes = umm.grid.get_mut(row, col).unwrap();

            for atom in prototypes.atoms.iter_mut() {
                if [
                    // "/turf/simulated/floor/exoplanet/barren",
                    "/turf/simulated/floor/exoplanet/dirt_konyang",
                ]
                .iter()
                .any(|p| atom.path.starts_with(p))
                {
                    let v = 16 - (*noise.get(row, col).unwrap() as i32);
                    let color = format!("{}{:X}{:X}{:X}{:X}{:X}{:X}", "#", v, v, v, v, v, v);

                    atom.vars
                        .insert("color".to_string(), dmmr::VarVal::String(color));
                }
            }
        }
    }

    for x in umm.grid.iter_mut() {
        if x.id.len() < 3 {
            x.id = format!("a{}", x.id);
        }
    }
    let repacked = pack(&umm);

    let result_str = print(&repacked);

    std::fs::write(result_path, result_str).unwrap();
}
