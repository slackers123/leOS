use generator::generate;
use parser::{parse, ASTRoot};
use state_machine::{EpsilonNFA, RunningEpsilonNFA};

mod bracketexpr;
mod generator;
mod parser;
mod state_machine;

pub struct Regex {
    inner: EpsilonNFA,
}

/// Create a regex from a pattern which can then be ran using [validate_regex]
///
/// ## Note:
/// This function will not produce errors just invalid (and potentially endless)
/// regular expressions
///
/// it follows the "standard" regex rules i.e. POSIX rules
/// https://en.wikibooks.org/wiki/Regular_Expressions/POSIX-Extended_Regular_Expressions
pub fn new_regex(src: &str) -> Regex {
    let src = src.chars().collect::<Vec<char>>();
    let parser_res = ASTRoot {
        orig_concats: parse(&src, &mut 0),
    };

    Regex {
        inner: generate(parser_res.to_expr()),
    }
}

pub fn validate_regex(regex: &Regex, src: &str) -> bool {
    let running = RunningEpsilonNFA::new(&regex.inner);
    running.validate(src)
}
