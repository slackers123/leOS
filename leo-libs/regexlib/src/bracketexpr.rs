use crate::{
    parser::handle_special_chars,
    state_machine::{BracketExpr, InnerBrackExpr},
};

pub fn parse_bracket_expr(src: &[char], idx: &mut usize) -> BracketExpr {
    assert!(src[*idx] == ']');
    let mut r_end = None;
    let mut res = BracketExpr {
        inverted: false,
        inner_be: vec![],
    };
    *idx -= 1;
    while src[*idx] != '[' {
        let next = src[*idx - 1];
        let next_escaped = if *idx > 1 {
            src[*idx - 2] == '\\'
        } else {
            false
        };
        let char = if next == '\\' {
            handle_special_chars(src[*idx]);
            *idx -= 2;
            continue;
        } else {
            src[*idx]
        };

        if next == '[' && char == '^' {
            res.inverted = true;
            break;
        }

        if r_end.is_none() {
            if next == '-' && !next_escaped {
                r_end = Some(char);
                *idx -= 2;
                continue;
            } else {
                res.inner_be.push(InnerBrackExpr::Char(char));
            }
        } else {
            res.inner_be
                .push(InnerBrackExpr::Range(char, r_end.unwrap()));
            r_end = None;
        }

        *idx -= 1;
    }
    res
}
