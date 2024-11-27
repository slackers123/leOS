use generator::generate;
use parser::Parser;
use state_machine::{EpsilonNFA, RunningEpsilonNFA};

mod bracketexpr;
mod generator;
mod parser;
mod state_machine;

pub fn new_regex_state_machine(src: &str) -> EpsilonNFA {
    let src = src.chars().collect::<Vec<char>>();
    let parser = Parser::new(&src);
    let parser_res = parser.parse();

    generate(parser_res.to_expr())
}

pub fn validate_regex(state_machine: EpsilonNFA, src: &str) -> bool {
    let running = RunningEpsilonNFA::new(&state_machine);
    running.validate(src)
}
