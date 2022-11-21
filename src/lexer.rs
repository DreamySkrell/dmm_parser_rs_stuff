use std::ops::Add;

#[derive(Debug, Clone)]
pub enum Token {
    Comment(String),

    PrototypeDefStart(String),
    AtomPath(String),

    VarInt((String, i32)),
    VarString((String, String)),
    VarList((String, Vec<i32>)),
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

fn str_to_int_list(s: &str) -> Vec<i32> {
    s.split(",").map(|i| i.parse().unwrap()).collect()
}

pub fn lexe(dmm: &str) -> Vec<(usize, Token)> {
    let mut tokens = Vec::<(usize, Token)>::new();
    let mut prototype_len = 99999;

    for (n, line) in dmm.lines().enumerate().map(|(n, l)| (n + 1, l)) {
        let token = {
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
                Token::RowDefStart(str_to_int_list(substr_between(line, "(", ")").into()))
            } else if line.contains(" = \"") {
                let name = substr_between(line, "\t", " = ");
                let val = substr_between(line, "\"", "\"");
                Token::VarString((name.into(), val.into()))
            } else if line.contains(" = list(") {
                let name = substr_between(line, "\t", " = list(");
                let val = substr_between(line, "list(", ")");
                Token::VarList((name.into(), str_to_int_list(val)))
            } else if line.contains(" = ") {
                let name = substr_between(line, "\t", " = ");
                let mut val: String =
                    line[line.find(" = ").map(|n| n + " = ".len()).unwrap()..].into();
                if val.ends_with(";") {
                    val.pop();
                }
                Token::VarInt((name.into(), val.parse::<i32>().unwrap()))
            } else if line.len() == prototype_len {
                Token::PrototypeId(line.into())
            } else if line.contains("},") {
                Token::VarEnd
            } else if line == "" {
                Token::EmptyLine
            } else {
                panic!("cannot lexe line: {}", n)
            }
        };
        //println!("{}: {:?}", n, token.clone());
        tokens.push((n, token));
    }
    tokens
}
