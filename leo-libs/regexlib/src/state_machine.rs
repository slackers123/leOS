use std::collections::HashMap;

pub type Transitions = Vec<Vec<(usize, Option<TransitionCondition>)>>;

/// conditions mainly based on POSIX-Extended Regular Expressions:
/// https://en.wikibooks.org/wiki/Regular_Expressions/POSIX-Extended_Regular_Expressions#Table_of_metacharacters
#[derive(Debug)]
pub enum TransitionCondition {
    Any,
    Char(char),
    BracketExpr(BracketExpr),
    StrStart,
    StrEnd,
}

impl TransitionCondition {
    pub fn check(&self, c: char, is_start: bool, is_end: bool) -> bool {
        match self {
            Self::Any => true,
            Self::Char(c1) => c == *c1,
            Self::BracketExpr(bracket_expr) => bracket_expr.check(c),
            Self::StrStart => is_start,
            Self::StrEnd => is_end,
        }
    }
}

/// A regex bracket expression.
#[derive(Debug, Clone, PartialEq)]
pub struct BracketExpr {
    /// a bracket expression is inverted when a ^ character is the first character in the
    /// bracket expression.
    pub inverted: bool,
    /// a bracket expression may be made up of multiple parts which can eiter be individual
    /// characters or ranges of characters
    pub inner_be: Vec<InnerBracketExpr>,
}

impl BracketExpr {
    /// checks if a
    pub fn check(&self, c: char) -> bool {
        let mut res = false;
        for inner in &self.inner_be {
            match inner {
                InnerBracketExpr::Char(c1) => {
                    if *c1 == c {
                        res = true;
                        break;
                    }
                }
                InnerBracketExpr::Range(c1, c2) => {
                    if (*c1..=*c2).contains(&c) {
                        res = true;
                        break;
                    }
                }
            }
        }

        self.inverted != res
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InnerBracketExpr {
    Char(char),
    Range(char, char),
}

/// (Epsilon) nondeterministic finite automaton
#[derive(Debug)]
pub struct EpsilonNFA {
    /// ### The outer Vec
    /// The source state so when trying to figure out which
    /// transitions can be made from e.g. state 5 use ```transitions[5]```
    ///
    /// ### The inner Vec
    /// The possible transitions from a state to new states.
    /// The ```usize``` is the new state and the ```Option<TransitionCondition>``` is
    /// the optional condition (None -> Epsilon condition)
    pub transitions: Transitions,
}

/// A running (epsilon) nondeterministic finite automaton
///
/// This is useful for reusing regexes since the parsing and NFA generation
/// take a lot longer than execution.
#[derive(Debug, Clone)]
pub struct RunningEpsilonNFA<'a> {
    state_machine: &'a EpsilonNFA,
    current_states: Vec<usize>,
}

impl<'a> RunningEpsilonNFA<'a> {
    pub fn new(state_machine: &'a EpsilonNFA) -> Self {
        let mut new_states = HashMap::new();
        Self::get_new_states(state_machine, 0, &mut new_states);
        Self {
            current_states: new_states.into_keys().collect(),
            state_machine,
        }
    }

    pub fn validate(mut self, string: &str, skip: usize) -> (bool, usize) {
        let mut chars_covered = 0;
        let mut early_exit_flag = false;
        let mut last_states = self.current_states.clone();
        for (i, c) in string.chars().enumerate().skip(skip) {
            last_states = self.current_states.clone();
            self.run_iteration(c, i == 0, i == string.len() - 1);
            chars_covered += 1;
            if self.current_states.is_empty() {
                early_exit_flag = true;
                break;
            }
        }
        if early_exit_flag && last_states.contains(&(self.state_machine.transitions.len() - 1)) {
            return (true, chars_covered - 1);
        }
        if self
            .current_states
            .contains(&(self.state_machine.transitions.len() - 1))
        {
            return (true, chars_covered);
        }
        (false, chars_covered)
    }

    pub fn run_iteration(&mut self, c: char, is_start: bool, is_end: bool) {
        let mut new_states = HashMap::new();
        for state in &self.current_states {
            for transition in &self.state_machine.transitions[*state] {
                if transition.1.is_none() && !self.current_states.contains(&transition.0)
                    || transition.1.is_some()
                        && transition.1.as_ref().unwrap().check(c, is_start, is_end)
                {
                    Self::get_new_states(self.state_machine, transition.0, &mut new_states);
                }
            }
        }

        self.current_states = new_states.into_keys().collect();
    }

    fn get_new_states(
        state_machine: &EpsilonNFA,
        state: usize,
        new_states: &mut HashMap<usize, ()>,
    ) {
        let r = new_states.insert(state, ());
        if r.is_some() {
            return;
        }
        for transition in &state_machine.transitions[state] {
            if transition.1.is_none() {
                Self::get_new_states(state_machine, transition.0, new_states);
            }
        }
    }
}
