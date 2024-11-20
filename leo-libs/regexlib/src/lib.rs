use generator::generate;
use parser::Parser;
use state_machine::{EpsilonNFA, RunningEpsilonNFA};

mod generator;
mod parser;
mod state_machine;

pub fn new_regex_state_machine(src: &str) -> EpsilonNFA {
    let src = src.chars().collect::<Vec<char>>();
    let parser = Parser::new(&src);
    let parser_res = dbg!(parser.parse());

    generate(parser_res.to_expr())
}

pub fn validate_regex(state_machine: EpsilonNFA, src: &str) -> bool {
    let running = RunningEpsilonNFA::new(&state_machine);
    running.validate(src)
}

// pub fn test_epsilon_state_machine() {
//     // let mut res_expr = None;
//     // pparse(
//     //     &"abc|def".chars().collect::<Vec<char>>(),
//     //     &mut 0,
//     //     &mut res_expr,
//     // );
//     // println!("{:#?}", res_expr.unwrap());
//     let src = "a(abc)*|def".chars().collect::<Vec<char>>();
//     let parser = Parser::new(&src);
//     let parser_res = parser.parse();

//     let state_machine = generate(parser_res.to_expr());
//     let src = "a";

//     let running = RunningEpsilonNFA::new(&state_machine);
//     println!("{src} is valid? {}", running.validate(src));
// }
