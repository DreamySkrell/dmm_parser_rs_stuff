use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::num::NonZeroU32;
use wfc::retry::*;
use wfc::*;
use wfc_image::*;

struct Forbid {
    pattern_ids: HashSet<PatternId>,
    offset: i32,
}

impl ForbidPattern for Forbid {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        let output_size = fi.wave_size();
        (0..(output_size.width() as i32))
            .map(|x| Coord::new(x, output_size.height() as i32 - self.offset as i32))
            .chain(
                (0..(output_size.width() as i32))
                    .map(|y| Coord::new(output_size.width() as i32 - self.offset as i32, y)),
            )
            .for_each(|coord| {
                self.pattern_ids.iter().for_each(|&pattern_id| {
                    fi.forbid_all_patterns_except(coord, pattern_id, rng)
                        .unwrap();
                });
            });
    }
}

pub fn generate(
    output_size: Size,
    pattern_size: u32,
    seed: u64,
    input_image: image::DynamicImage,
    // output_path: String,
    orientations: &'static [orientation::Orientation],
    retries: usize,
    allow_corner: bool,
) -> Option<image::DynamicImage> {
    println!("{}", seed);
    let mut rng = rand::thread_rng();
    let mut image_patterns = ImagePatterns::new(
        &input_image,
        NonZeroU32::new(pattern_size).expect("pattern size may not be zero"),
        orientations,
    );
    let input_size = image_patterns.grid().size();
    let bottom_right_offset = pattern_size - (pattern_size / 2);
    let id_grid = image_patterns.id_grid();
    let bottom_right_coord = Coord::new(
        input_size.width() as i32 - bottom_right_offset as i32,
        input_size.height() as i32 - bottom_right_offset as i32,
    );
    let bottom_right_ids = id_grid
        .get_checked(bottom_right_coord)
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    if !allow_corner {
        bottom_right_ids.iter().for_each(|&pattern_id| {
            image_patterns.pattern_mut(pattern_id).clear_count();
        });
    }
    let global_stats = image_patterns.global_stats();
    let mut wave = Wave::new(output_size);
    let mut context = Context::new();
    let result = {
        let forbid = Forbid {
            pattern_ids: bottom_right_ids,
            offset: bottom_right_offset as i32,
        };
        let mut run =
            RunBorrow::new_forbid(&mut context, &mut wave, &global_stats, forbid, &mut rng);
        run.collapse_retrying(NumTimes(retries), &mut rng)
    };
    match result {
        Err(_) => {
            eprintln!("Too many contradictions!");
            None
        }
        Ok(()) => {
            // image_patterns
            //     .image_from_wave(&wave)
            //     .save(output_path)
            //     .unwrap();
            Some(image_patterns.image_from_wave(&wave))
        }
    }
}
