use crate::{bracketexpr::parse_bracket_expr, generator::Expr, state_machine::BracketExpr};

#[derive(Debug, Clone, PartialEq)]
pub struct ASTRoot {
    pub orig_concats: Vec<ASTNode>,
}

impl ASTRoot {
    pub fn into_expr(self) -> Expr {
        Self::node_vec_concat(self.orig_concats)
    }

    fn node_to_expr(node: ASTNode) -> Expr {
        match node {
            ASTNode::Any => Expr::Any,
            ASTNode::Or { lhs, rhs } => Expr::Union {
                s: Box::new(Self::node_vec_concat(rhs)),
                t: Box::new(Self::node_vec_concat(lhs)),
            },
            ASTNode::StrEnd => Expr::StrEnd,
            ASTNode::Char(c) => Expr::Char(c),
            ASTNode::StrStart => Expr::StrStart,
            ASTNode::BracketExpr(b) => Expr::BracketExpr(b),
            ASTNode::InRange { inner, start, end } => todo!(),
            ASTNode::Subexpr { inner } => Self::node_vec_concat(inner),
            ASTNode::OneOrMore { inner } => Expr::Concat {
                // a+ <=> aa*
                s: Box::new(Self::node_to_expr(*inner.clone())),
                t: Box::new(Expr::KleeneStar {
                    s: Box::new(Self::node_to_expr(*inner)),
                }),
            },
            ASTNode::ZeroOrOne { inner } => Expr::Union {
                // a? <=> a|nothing
                s: Box::new(Expr::None),
                t: Box::new(Self::node_to_expr(*inner)),
            },
            ASTNode::ZeroOrMore { inner } => Expr::KleeneStar {
                s: Box::new(Self::node_to_expr(*inner)),
            },
        }
    }

    fn node_vec_concat(nodes: Vec<ASTNode>) -> Expr {
        let mut start = Self::node_to_expr(nodes[0].clone());
        for node in nodes.into_iter().skip(1) {
            Self::concat(&mut start, Self::node_to_expr(node));
        }
        start
    }

    fn concat(orig: &mut Expr, new: Expr) {
        *orig = Expr::Concat {
            s: Box::new(orig.clone()),
            t: Box::new(new),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Char(char),
    Or {
        lhs: Vec<ASTNode>,
        rhs: Vec<ASTNode>,
    },
    Any,
    BracketExpr(BracketExpr),
    StrStart,
    StrEnd,
    Subexpr {
        inner: Vec<ASTNode>,
    },
    ZeroOrMore {
        inner: Box<ASTNode>,
    },
    OneOrMore {
        inner: Box<ASTNode>,
    },
    ZeroOrOne {
        inner: Box<ASTNode>,
    },
    InRange {
        inner: Box<ASTNode>,
        start: usize,
        end: usize,
    },
}

fn handle_regex_special_chars(c: char) -> ASTNode {
    match c {
        '(' | ')' | '[' | ']' | '.' | '*' | '?' | '+' | '|' | '^' | '$' | '\\' => ASTNode::Char(c),
        't' => ASTNode::Char('\t'),
        'n' => ASTNode::Char('\n'),
        'r' => ASTNode::Char('\r'),
        _ => ASTNode::Char(handle_special_chars(c)),
    }
}
pub fn handle_special_chars(c: char) -> char {
    match c {
        't' => '\t',
        'n' => '\n',
        'r' => '\r',
        '-' => '-',
        _ => {
            panic!("unknown escape character: {c:?}");
        }
    }
}

pub fn parse(src: &[char], index: &mut usize) -> Vec<ASTNode> {
    let mut nodes: Vec<ASTNode> = Vec::new();

    while *index < src.len() {
        let c = src[*index];
        match c {
            '*' => {
                if let Some(last) = nodes.last_mut() {
                    *last = ASTNode::ZeroOrMore {
                        inner: Box::new(last.clone()),
                    };
                }
            }
            '+' => {
                if let Some(last) = nodes.last_mut() {
                    *last = ASTNode::OneOrMore {
                        inner: Box::new(last.clone()),
                    };
                }
            }
            '?' => {
                if let Some(last) = nodes.last_mut() {
                    *last = ASTNode::ZeroOrOne {
                        inner: Box::new(last.clone()),
                    };
                }
            }
            '|' => {
                *index += 1;
                nodes = vec![ASTNode::Or {
                    lhs: nodes,
                    rhs: parse(src, index),
                }];
                break;
            }
            '(' => {
                *index += 1;
                let inner = parse(src, index);
                nodes.push(ASTNode::Subexpr { inner });
            }
            ')' => {
                break;
            }
            '[' => {
                nodes.push(ASTNode::BracketExpr(parse_bracket_expr(src, index)));
            }
            '\\' => {
                *index += 1;
                let escaped = src[*index];
                nodes.push(handle_regex_special_chars(escaped));
            }
            '.' => nodes.push(ASTNode::Any),
            '^' => nodes.push(ASTNode::StrStart),
            '$' => nodes.push(ASTNode::StrEnd),
            '{' => {
                if let Some(last) = nodes.last_mut() {
                    *last = parse_range(src, index, Box::new(last.clone()));
                }
            }
            _ => nodes.push(ASTNode::Char(c)),
        }
        *index += 1;
    }
    nodes
}

pub fn parse_range(src: &[char], idx: &mut usize, inner: Box<ASTNode>) -> ASTNode {
    assert!(src[*idx] == '{');

    let mut start = 0;
    let mut end = 0;

    let mut first = true;

    while src[*idx] != '}' {
        let cur = src[*idx];
        if cur == ',' {
            first = false;
            continue;
        }

        if first {
            start *= 10;
            start += (cur as u8) as usize - 48;
        } else {
            end *= 10;
            end += (cur as u8) as usize - 48;
        }
    }
    ASTNode::InRange { inner, start, end }
}
