use std::ops::Add;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),

    PrototypeDefStart(String),
    AtomPath(String),

    VarInt((String, f64)),
    VarNull((String, ())),
    VarString((String, String)),
    VarIcon((String, String)),
    VarPath((String, String)),
    VarList((String, Vec<i32>)),
    VarListString((String, Vec<String>)),
    VarListStringAssoc((String, Vec<(String, String)>)),
    VarListPath((String, Vec<String>)),
    VarEnd,

    RowDefStart(Vec<i32>),
    PrototypeId(String),
    RowDefEnd,

    EmptyLine,

    Err,
}

fn substr_between<'a>(s: &'a str, start_pat: &str, end_pat: &str) -> &'a str {
    let start_bytes = s
        .find(start_pat)
        .map(|n| n + start_pat.len())
        .unwrap_or(0)
        .clamp(0, s.len());
    let end_bytes = s[start_bytes..]
        .find(end_pat)
        .unwrap_or(s.len())
        .add(start_bytes)
        .clamp(0, s.len());
    let result = &s[start_bytes..end_bytes];
    result
}

fn str_to_int_list(s: &str, line: &str) -> Vec<i32> {
    if s == "" {
        [].into()
    } else {
        s.split(",")
            .map(|i| i.parse().expect(&format!("line: {line}")))
            .collect()
    }
}

pub fn lexe(dmm: &str) -> Vec<(usize, Token)> {
    let mut tokens = Vec::<(usize, Token)>::new();
    let mut prototype_len = 99999;

    for (n, line) in dmm.lines().enumerate().map(|(n, l)| (n + 1, l)) {
        let token = {
            // dbg!("", n, line);
            if line.starts_with("//") {
                Token::Comment(line.chars().skip(2).collect())
            } else if line.starts_with("\"}") {
                Token::RowDefEnd
            } else if line.ends_with(" = (") {
                let prototype_id = substr_between(line, "\"", "\"");
                prototype_len = prototype_id.len();
                Token::PrototypeDefStart(prototype_id.into())
            } else if line.starts_with("/") {
                let mut p = line.to_string();
                p.pop();
                Token::AtomPath(p)
            } else if line.contains(") = {\"") {
                Token::RowDefStart(str_to_int_list(substr_between(line, "(", ")").into(), line))
            } else if line.contains(" = \"") {
                let name = substr_between(line, "\t", " = ");
                let val = substr_between(line, "\"", "\"");
                Token::VarString((name.into(), val.into()))
            } else if line.contains(" = '") {
                let name = substr_between(line, "\t", " = ");
                let val: &str = substr_between(line, "'", "'");
                Token::VarIcon((name.into(), val.into()))
            } else if line.contains(" = /") {
                let name = substr_between(line, "\t", " = ");
                let val = substr_between(line, " = ", ";");
                Token::VarPath((name.into(), val.into()))
            } else if line.contains(" = list(\"") && line.contains("\"=\"") {
                let name = substr_between(line, "\t", " = list(");
                let val = substr_between(line, "list(", ")")
                    .split(",")
                    .map(|s| {
                        s.split('=')
                            .map(|s| substr_between(s, "\"", "\""))
                            .map(|s| s.to_string())
                            .next_tuple::<(String, String)>()
                            .unwrap()
                    })
                    .collect_vec();
                Token::VarListStringAssoc((name.into(), val))
            } else if line.contains(" = list(\"") {
                let name = substr_between(line, "\t", " = list(");
                let val = substr_between(line, "list(", ")")
                    .split(",")
                    .map(|s| substr_between(s, "\"", "\""))
                    .map(|s| s.to_string())
                    .collect_vec();
                Token::VarListString((name.into(), val))
            } else if line.contains(" = list(/") {
                let name = substr_between(line, "\t", " = list(");
                let val = substr_between(line, "list(", ")")
                    .split(",")
                    // .map(|s| substr_between(s, "\"", "\""))
                    .map(|s| s.to_string())
                    .collect_vec();
                Token::VarListPath((name.into(), val))
            } else if line.contains(" = list(") {
                let name = substr_between(line, "\t", " = list(");
                let val = substr_between(line, "list(", ")");
                Token::VarList((name.into(), str_to_int_list(val, line)))
            } else if line.contains(" = null") {
                let name = substr_between(line, "\t", " = ");
                Token::VarNull((name.into(), ()))
            } else if line.contains(" = ") {
                let name = substr_between(line, "\t", " = ");
                let mut val: String =
                    line[line.find(" = ").map(|n| n + " = ".len()).unwrap()..].into();
                if val.ends_with(";") {
                    val.pop();
                }
                Token::VarInt((
                    name.into(),
                    val.parse::<f64>().expect(&format!("line: {line}")),
                ))
            } else if line.contains("},") {
                Token::VarEnd
            } else if line.contains("})") {
                Token::VarEnd
            } else if line.len() == prototype_len {
                Token::PrototypeId(line.into())
            } else if line == "" {
                Token::EmptyLine
            } else {
                panic!("cannot lexe line: {}", n)
            }
        };
        // println!("{}: {:?}", n, token.clone());
        tokens.push((n, token));
    }
    tokens
}
