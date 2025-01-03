//! https://en.wikibooks.org/wiki/Regular_Expressions/POSIX-Extended_Regular_Expressions#Table_of_metacharacters
//!
//! s?      <=> s | ε
//! s+      <=> ss*

use crate::state_machine::{BracketExpr, EpsilonNFA, TransitionCondition, Transitions};

#[derive(Debug, Clone)]
pub enum Expr {
    None,
    Char(char),
    Union { s: Box<Expr>, t: Box<Expr> },
    Concat { s: Box<Expr>, t: Box<Expr> },
    KleeneStar { s: Box<Expr> },
    BracketExpr(BracketExpr),
    StrStart,
    StrEnd,
    Any,
}

impl Expr {
    pub fn generate_transitions(self, transitions: &mut Transitions, states_cnt: &mut usize) {
        fn new_state(transitions: &mut Transitions, states_cnt: &mut usize) {
            transitions.push(vec![]);
            *states_cnt += 1;
        }

        match self {
            Expr::None => {
                transitions[*states_cnt].push((*states_cnt + 1, None));
                new_state(transitions, states_cnt);
            }
            Expr::Char(c) => {
                transitions[*states_cnt]
                    .push((*states_cnt + 1, Some(TransitionCondition::Char(c))));
                new_state(transitions, states_cnt);
            }
            Expr::Union { s, t } => {
                let orig = *states_cnt;

                transitions[orig].push((*states_cnt + 1, None));
                new_state(transitions, states_cnt);
                s.generate_transitions(transitions, states_cnt);
                let s_out = *states_cnt;

                transitions[orig].push((*states_cnt + 1, None));
                new_state(transitions, states_cnt);
                t.generate_transitions(transitions, states_cnt);
                let t_out = *states_cnt;

                transitions[s_out].push((*states_cnt + 1, None));
                transitions[t_out].push((*states_cnt + 1, None));

                new_state(transitions, states_cnt);
            }
            Expr::Concat { s, t } => {
                s.generate_transitions(transitions, states_cnt);
                t.generate_transitions(transitions, states_cnt);
            }
            Expr::KleeneStar { s } => {
                let orig = *states_cnt;

                transitions[orig].push((*states_cnt + 1, None));
                new_state(transitions, states_cnt);
                let s_in = *states_cnt;
                s.generate_transitions(transitions, states_cnt);
                let s_out = *states_cnt;

                transitions[s_out].push((s_in, None));

                new_state(transitions, states_cnt);

                let out = *states_cnt;
                transitions[s_out].push((out, None));
                transitions[orig].push((out, None));
            }
            Expr::Any => {
                transitions[*states_cnt].push((*states_cnt + 1, Some(TransitionCondition::Any)));
                new_state(transitions, states_cnt);
            }
            Expr::BracketExpr(b) => {
                transitions[*states_cnt]
                    .push((*states_cnt + 1, Some(TransitionCondition::BracketExpr(b))));
                new_state(transitions, states_cnt);
            }
            Expr::StrEnd => {
                transitions[*states_cnt].push((*states_cnt + 1, Some(TransitionCondition::StrEnd)));
                new_state(transitions, states_cnt);
            }
            Expr::StrStart => {
                transitions[*states_cnt]
                    .push((*states_cnt + 1, Some(TransitionCondition::StrStart)));
                new_state(transitions, states_cnt);
            }
        }
    }
}

pub fn generate(src: Expr) -> EpsilonNFA {
    let mut transitions: Transitions = vec![vec![]];
    let mut states_cnt = 0;
    src.generate_transitions(&mut transitions, &mut states_cnt);

    EpsilonNFA { transitions }
}
