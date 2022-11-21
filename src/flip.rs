use crate::*;

#[test]
fn flip() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        let original = std::fs::read_to_string(&path).unwrap();

        let mut parsed = parse(&original);

        {
            for row in &mut parsed.rows {
                row.tiles.reverse();
            }
        }

        let printed = print(&parsed);

        std::fs::write(
            dbg!(format!(
                "data_flip/{}",
                path.file_name().unwrap().to_str().unwrap()
            )),
            printed,
        )
        .unwrap();
    }
}
