#![allow(unused_variables)]

use std::{collections::HashMap, num::NonZeroU32};

use itertools::Itertools;
use rand::Rng;
use wfc::Coord;

use crate::dmmr::Umm;

mod anchor;
mod anchor_walls_outside;

pub fn generate_wfc_image() {
    let template_map_name = "D:/Git/Aurora.3/maps/____wfc_template.dmm";
    let template_path: std::path::PathBuf = format!("{template_map_name}").into();
    println!("template map: {}", template_path.to_str().unwrap());
    let result_path: std::path::PathBuf = "D:/Git/Aurora.3/maps/____wfc_output.dmm".into();

    let template_map_str = std::fs::read_to_string(&template_path).unwrap();
    let template_parsed = crate::dmmr::parse(&template_map_str);
    let template_umm = crate::dmmr::unpack(&template_parsed);

    // str id to num id
    let mut prototype_strid_to_numid = HashMap::<String, usize>::new();
    let mut prototype_numid_to_strid = HashMap::<usize, String>::new();
    for (num_id, prototype) in template_parsed.prototypes.iter().enumerate() {
        prototype_strid_to_numid.insert(prototype.id.clone(), num_id);
        prototype_numid_to_strid.insert(num_id, prototype.id.clone());
        println!("qq: {}", num_id);
    }

    println!("1");
    // let input_image = image::open("W:/aaa.png").unwrap();
    let input_image = image::DynamicImage::ImageRgba8(
        image::RgbaImage::from_vec(
            template_umm.grid.cols() as u32,
            template_umm.grid.rows() as u32,
            template_umm
                .grid
                .iter()
                .map(|x| [*prototype_strid_to_numid.get(&x.id).unwrap() as u8, 0, 0, 0])
                .flatten()
                .collect_vec(),
        )
        .unwrap(),
    );
    println!("2");
    // let output_image = wfc_image::generate_image(
    //     &input_image,
    //     NonZeroU32::new(2).unwrap(),
    //     wfc::Size::new(16, 16),
    //     &[wfc::Orientation::Original], // &wfc::orientation::ALL,
    //     wfc_image::wrap::WrapNone,
    //     wfc::ForbidNothing,
    //     wfc_image::retry::NumTimes(999999),
    // )
    // .unwrap();
    // let output_image = anchor::generate(
    //     wfc::Size::new(16, 16),
    //     2,
    //     rand::thread_rng().gen(),
    //     input_image,
    //     &[wfc::Orientation::Original],
    //     999999,
    //     false,
    // )
    // .unwrap();
    let output_image = anchor_walls_outside::generate(
        wfc::Size::new(16, 16),
        2,
        rand::thread_rng().gen(),
        input_image,
        &[wfc::Orientation::Original],
        999999,
        false,
    )
    .unwrap();
    output_image.save("W:/bbb.bmp").unwrap();
    println!("3");

    let output_vec: Vec<u8> = output_image
        .as_rgba8()
        .unwrap()
        .pixels()
        .map(|px| *px)
        .map(|px| px[0])
        .collect_vec();
    println!("output vec len: {}", output_vec.len());
    let mut output_umm = Umm {
        comment:
            "//MAP CONVERTED BY dmm2tgm.py THIS HEADER COMMENT PREVENTS RECONVERSION, DO NOT REMOVE"
                .into(),
        grid: grid::Grid::new(16, 16),
    };
    for x in 0..16 {
        for y in 0..16 {
            let tile = output_umm.grid.get_mut(x, y).unwrap();
            let prototype_numid = *output_vec.get(x + y * 16).unwrap() as usize;
            println!("ugh: {} ({},{})", prototype_numid, x, y);
            let prototype_strid = prototype_numid_to_strid
                .get(&prototype_numid)
                .unwrap_or(prototype_numid_to_strid.get(&0).unwrap())
                .clone();
            tile.id = prototype_strid.clone();
            tile.atoms = template_parsed
                .prototypes
                .iter()
                .find_or_first(|p| p.id == prototype_strid)
                .unwrap()
                .atoms
                .clone();
        }
    }

    let output_repacked = crate::dmmr::pack(&output_umm);
    let output_str = crate::dmmr::print(&output_repacked);
    std::fs::write(result_path, output_str).unwrap();
}
