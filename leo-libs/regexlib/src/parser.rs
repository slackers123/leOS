//! abc|def
//! Char a
//!
//! Concat
//! -> Char a
//! -> Char b
//!
//! Concat
//! -> Concat
//!     -> Char a
//!     -> Char b
//! -> Char c
//!
//! Union
//! -> Concat
//!     -> Concat
//!         -> Char a
//!         -> Char b
//!     -> Char c
//! -> Char d
//!
//! ---
//!
//! Union
//! -> Concat
//!     -> Char a
//!     -> Concat
//!         -> Char b
//!         -> Char c
//! -> Concat
//!     -> Char d
//!     -> Concat
//!         -> Char e
//!         -> char f
//!
//! ---
//!
//! |abc
//! Union
//! -> None
//! -> Concat
//!     -> Char a
//!     -> Concat
//!         -> Char b
//!         -> Char c
//!
//!
//! a(abc)*|def
//!
//! av:
//! f
//! e
//! d
//!
//! new outer av
//! av:
//! union -rhs> fed
//!
//! av is now union lhs
//! av:
//! zeroOrMore -> subexpr
//!
//! av is now subexpr inner
//! av:
//! c
//! b
//! a
//!
//! go up in av tree
//! av:
//! zeroOrMore -> subexpr -> dba
//! a
//!
//! Union
//! -> Concat
//!     -> a
//!     -> ZeroOrMore
//!         -> Subexpr
//!             -> abc
//! -> def
//!
//!
//!
//! cba
//! Concat
//! -> a
//! -> Concat
//!     -> b
//!     -> c

use crate::{generator::Expr, state_machine::BracketExpr};

#[derive(Debug, Clone)]
pub struct ASTRoot {
    pub orig_concats: Vec<ASTNode>,
}

impl ASTRoot {
    pub fn to_expr(self) -> Expr {
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
            ASTNode::NotSet => panic!("somehow a NotSet has leaked into the expr generation."),
            ASTNode::Char(c) => Expr::Char(c),
            ASTNode::StrStart => Expr::StrStart,
            ASTNode::AtMost { inner: _, end: _ } => {
                todo!()
            }
            ASTNode::AtLeast { inner: _, start: _ } => {
                todo!()
            }
            ASTNode::BracketExpr(b) => Expr::BracketExpr(b),
            ASTNode::InRange {
                inner: _,
                start: _,
                end: _,
            } => todo!(),
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
            s: Box::new(new),
            t: Box::new(orig.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    NotSet,
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
    AtLeast {
        inner: Box<ASTNode>,
        start: usize,
    },
    AtMost {
        inner: Box<ASTNode>,
        end: usize,
    },
}

pub struct Parser<'a> {
    pub src: &'a [char],
    idx: usize,
    outer_vec: Vec<ASTNode>,
    vec_depth: usize,
    union_depths: Vec<usize>,
    /// currently parsing *, +, ?, etc.
    in_single: bool,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a [char]) -> Parser<'a> {
        Parser {
            idx: src.len() - 1,
            src,
            outer_vec: Vec::new(),
            vec_depth: 0,
            union_depths: Vec::new(),
            in_single: false,
        }
    }

    fn get_av(&mut self) -> &mut Vec<ASTNode> {
        fn get_new_curr<'b>(last: &'b mut ASTNode) -> &'b mut Vec<ASTNode> {
            match last {
                ASTNode::Or { lhs, rhs: _ } => lhs,
                ASTNode::Subexpr { inner } => inner,
                ASTNode::ZeroOrMore { inner } => get_new_curr(inner),
                ASTNode::OneOrMore { inner } => get_new_curr(inner),
                ASTNode::ZeroOrOne { inner } => get_new_curr(inner),
                ASTNode::InRange {
                    inner,
                    start: _,
                    end: _,
                } => get_new_curr(inner),
                ASTNode::AtLeast { inner, start: _ } => get_new_curr(inner),
                ASTNode::AtMost { inner, end: _ } => get_new_curr(inner),
                _ => panic!("vec depth not reachable"),
            }
        }

        let mut curr = &mut self.outer_vec;
        let mut d = 0;
        while d < self.vec_depth {
            if let Some(last) = curr.last_mut() {
                curr = get_new_curr(last);
                d += 1;
            } else {
                panic!("vec depth not reachable");
            }
        }
        curr
    }

    pub fn parse(mut self) -> ASTRoot {
        fn add_to_deepest_single(t_node: &mut ASTNode, new_node: ASTNode) {
            match t_node {
                ASTNode::ZeroOrMore { ref mut inner } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                ASTNode::OneOrMore { ref mut inner } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                ASTNode::ZeroOrOne { ref mut inner } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                ASTNode::InRange {
                    ref mut inner,
                    start: _,
                    end: _,
                } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                ASTNode::AtLeast {
                    ref mut inner,
                    start: _,
                } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                ASTNode::AtMost {
                    ref mut inner,
                    end: _,
                } => {
                    if *inner.as_ref() == ASTNode::NotSet {
                        *inner = Box::new(new_node);
                    } else {
                        add_to_deepest_single(inner, new_node);
                    }
                }
                _ => panic!("not a single"),
            }
        }

        loop {
            let i = self.idx;
            let c = self.src[i];
            if i > 0 && self.src[i - 1] == '\\' {
                todo!("handle backslash");
            }
            let p_in_single = self.in_single;
            let av = self.get_av();

            match c {
                '|' => {
                    if p_in_single {
                        panic!("| can not be followed by *, ?, +, etc.");
                    }
                    *av = vec![ASTNode::Or {
                        lhs: vec![],
                        rhs: av.clone(),
                    }];
                    self.vec_depth += 1;
                    self.union_depths.push(self.vec_depth);
                }
                ']' => {
                    todo!("parse box expr")
                }
                '.' => {
                    if p_in_single {
                        add_to_deepest_single(av.last_mut().unwrap(), ASTNode::Any);
                        self.in_single = false;
                    } else {
                        av.push(ASTNode::Any);
                    }
                }
                '^' => {
                    if p_in_single {
                        panic!("^ cannot be followed by *, ?, +, etc.")
                    }
                    av.push(ASTNode::StrStart);
                }
                '$' => {
                    if p_in_single {
                        panic!("^ cannot be followed by *, ?, +, etc.")
                    }
                    av.push(ASTNode::StrEnd);
                }
                '\\' => {
                    unreachable!(
                        "should not be reachable because of stuff in the begining of this function"
                    )
                }
                '*' => {
                    if p_in_single {
                        panic!("* cannot be followed by *, ?, +, etc.")
                    }
                    av.push(ASTNode::ZeroOrMore {
                        inner: Box::new(ASTNode::NotSet),
                    });
                    self.in_single = true;
                }
                '?' => {
                    if p_in_single {
                        panic!("? cannot be followed by *, ?, +, etc.")
                    }
                    av.push(ASTNode::ZeroOrOne {
                        inner: Box::new(ASTNode::NotSet),
                    });
                    self.in_single = true;
                }
                '{' | '}' => {
                    todo!("ranges")
                }
                '(' => {
                    self.vec_depth -= 1;
                }
                ')' => {
                    if p_in_single {
                        add_to_deepest_single(
                            av.last_mut().unwrap(),
                            ASTNode::Subexpr { inner: vec![] },
                        );
                        self.in_single = false;
                    } else {
                        av.push(ASTNode::Subexpr { inner: vec![] });
                    }
                    self.vec_depth += 1;
                }
                _ => {
                    if p_in_single {
                        add_to_deepest_single(av.last_mut().unwrap(), ASTNode::Char(c));
                        self.in_single = false;
                    } else {
                        av.push(ASTNode::Char(c));
                    }
                }
            }

            if i == 0 {
                break;
            }
            self.idx -= 1;
        }

        ASTRoot {
            orig_concats: self.outer_vec,
        }
    }
}
