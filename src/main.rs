#![allow(dead_code)]
#![allow(unused_imports)]

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

pub struct Atom {
    path: String,
}

pub struct Prototype {
    id: String,
    atoms: Vec<Atom>,
}

pub struct Row {
    coords: Vec<i32>,
    tiles: Vec<String>,
}

pub struct Dmm {
    comment: String,
    prototypes: Vec<Prototype>,
    rows: Vec<Row>,
}

fn parse(dmm: &str) -> Dmm {
    parser::DmmParser::new().parse(&dmm).unwrap()
}

fn newline() -> &'static str {
    "\r\n"
}

fn print(dmm: &Dmm) -> String {
    let mut s = String::new();

    // comment
    s.push_str(&format!("{}{}", dmm.comment, newline()));

    // prototypes
    for proto in &dmm.prototypes {
        s.push_str(&format!("\"{}\" = (", proto.id));
        for (i, atom) in proto.atoms.iter().enumerate() {
            s.push_str(&format!("{}", newline()));
            s.push_str(&format!("{}", atom.path));
            if i < proto.atoms.len() - 1 {
                s.push_str(&format!(","));
            }
        }
        s.push_str(&format!("){}", newline()));
    }

    // break
    s.push_str(&format!("{}", newline()));

    // rows
    for row in &dmm.rows {
        s.push_str(&format!(
            "({},{},{}) = {{\"{}",
            row.coords[0],
            row.coords[1],
            row.coords[2],
            newline(),
        ));
        for tile in &row.tiles {
            s.push_str(&format!("{}{}", tile, newline()));
        }
        s.push_str(&format!("\"}}{}", newline()));
    }

    // done
    s
}

#[test]
fn sanity() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let _ = std::fs::read_to_string(dmm.unwrap().path()).unwrap();
    }
}

#[test]
fn parse_compare() {
    for dmm in std::fs::read_dir("data").unwrap() {
        let path = dmm.unwrap().path();
        println!("-- {}", path.to_str().unwrap());
        let original = std::fs::read_to_string(path).unwrap();
        println!("   read");
        let parsed = parse(&original);
        println!("   parsed");
        let printed = print(&parsed);
        println!("   printed");
        println!("{}", original);
        println!("{}", printed);
        assert_eq!(original, printed);
        println!("   ok");
    }
}

fn main() {
    //let dmm = std::fs::read_to_string("data/x.dmm").unwrap();
    //let dmm = parser::DmmParser::new().parse(&dmm).unwrap();
}
