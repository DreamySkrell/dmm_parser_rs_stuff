#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(illegal_floating_point_literal_pattern)]

use crate::dmmr::*;
use crate::*;

pub fn remap() {
    let map_dir = "D:/Git/Aurora.3.kyres1/maps/sccv_horizon".to_string();
    let map_name = "sccv_horizon-2_deck_2";
    let origin_path: std::path::PathBuf = format!("{map_dir}/{map_name}.dmm").into();
    let parsed_path: std::path::PathBuf = format!("{map_dir}/{map_name}_p.dmm").into();
    let result_path: std::path::PathBuf = format!("{map_dir}/{map_name}_r.dmm").into();

    let origin_map_str = std::fs::read_to_string(&origin_path).unwrap();
    let mut parsed = parse(&origin_map_str);

    let parsed_str = print(&parsed);
    std::fs::write(parsed_path, parsed_str.clone()).unwrap();
    for (i, diff) in diff::lines(&origin_map_str, &parsed_str).iter().enumerate() {
        match diff {
            diff::Result::Left(l) => println!("{} diff - : {}", i, l),
            diff::Result::Both(l, r) => {
                assert_eq!(l, r);
                //println!("{} diff   : {}", i, l);
            }
            diff::Result::Right(r) => println!("{} diff + : {}", i, r),
        }
    }
    assert!(origin_map_str == parsed_str);

    for prototype in &mut parsed.prototypes {

        //
    } // for prototype in &mut parsed.prototypes

    let result_str = print(&parsed);

    std::fs::write(result_path, result_str).unwrap();
}
