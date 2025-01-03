use generator::generate;
use parser::{parse, ASTRoot};
use state_machine::{EpsilonNFA, RunningEpsilonNFA};

mod bracketexpr;
mod generator;
mod parser;
mod state_machine;

/// A regular expression.
///
/// It can be created using the [Regex::new] function and a string can be matched against it by using the
/// [Regex::validate] function.
pub struct Regex {
    inner: EpsilonNFA,
}

impl Regex {
    /// Create a regex from a pattern which can then be run using [Regex::validate]
    ///
    /// ## Note:
    /// This function will not produce errors just invalid (and more likely endless)
    /// NFAs which will usually just result in a stack overflow from uncapped recursion.
    ///
    /// it follows the "standard" regex rules i.e. POSIX rules
    /// https://en.wikibooks.org/wiki/Regular_Expressions/POSIX-Extended_Regular_Expressions
    pub fn new(src: &str) -> Self {
        let src = src.chars().collect::<Vec<char>>();
        let parser_res = ASTRoot {
            orig_concats: parse(&src, &mut 0),
        };

        Regex {
            inner: generate(parser_res.into_expr()),
        }
    }

    /// Validates a string using the regex
    pub fn validate(&self, src: &str) -> bool {
        let running = RunningEpsilonNFA::new(&self.inner);
        let res = running.validate(src, 0);

        res.0 && res.1 == src.len()
    }

    /// finds all the matches in the input string
    pub fn find_matches(&self, src: &str) -> Vec<(usize, usize)> {
        let running = RunningEpsilonNFA::new(&self.inner);

        let mut matches = vec![];
        let mut index = 0;
        while index < src.len() {
            let res = running.clone().validate(src, index);

            if res.0 {
                matches.push((index, index + res.1));
            }
            index += res.1;
        }

        matches
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn union() {
        let r = Regex::new(r"(abc|def)|ghi");

        assert!(r.validate("abc"));
        assert!(r.validate("def"));
        assert!(r.validate("ghi"));

        assert!(!r.validate("xyz"));
    }

    #[test]
    fn concat() {
        let r = Regex::new(r"abc");

        assert!(r.validate("abc"));

        assert!(!r.validate("xyz"));
    }

    #[test]
    fn kleene_star() {
        let r = Regex::new(r"ab*c");

        assert!(r.validate("ac"));
        assert!(r.validate("abc"));
        assert!(r.validate("abbbc"));

        assert!(!r.validate("ab"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn kleene_plus() {
        let r = Regex::new(r"ab+c");

        assert!(r.validate("abc"));
        assert!(r.validate("abbbc"));

        assert!(!r.validate("ac"));
        assert!(!r.validate("ab"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn optional() {
        let r = Regex::new(r"ab?c");

        assert!(r.validate("abc"));
        assert!(r.validate("ac"));

        assert!(!r.validate("ab"));
        assert!(!r.validate("abbbc"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn sub_expr() {
        let r = Regex::new(r"(abc|def)ghi");

        assert!(r.validate("abcghi"));
        assert!(r.validate("defghi"));

        assert!(!r.validate("ghi"));
        assert!(!r.validate("abcdefghi"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn wildcard() {
        let r = Regex::new(r"a.c");

        assert!(r.validate("abc"));
        assert!(r.validate("adc"));

        assert!(!r.validate("ac"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn bracket_expr() {
        let r = Regex::new(r"[abch-z]");

        assert!(r.validate("a"));
        assert!(r.validate("b"));
        assert!(r.validate("c"));
        assert!(r.validate("s"));
        assert!(r.validate("z"));

        assert!(!r.validate("f"));
        assert!(!r.validate("xyz"));
    }

    #[test]
    fn inv_bracket_expr() {
        let r = Regex::new(r"[^abch-z]");

        assert!(!r.validate("a"));
        assert!(!r.validate("b"));
        assert!(!r.validate("c"));
        assert!(!r.validate("s"));
        assert!(!r.validate("z"));
        assert!(!r.validate("xyz"));

        assert!(r.validate("f"));
        assert!(r.validate("g"));
    }

    #[test]
    fn email() {
        let r = Regex::new(r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9.-]+");

        assert!(!r.validate("abc"));
        assert!(!r.validate("severin.gebesmair"));
        assert!(!r.validate("severin.gebesmair@gmail"));

        assert!(r.validate("severin.gebesmair@gmail.com"));
    }

    #[test]
    fn lorem_ipsum_mates_from_doc() {
        let r = Regex::new("[abc]+");

        let matches = r.find_matches("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.");

        assert_eq!(
            matches,
            // These results are from regex101.com using the pcre2 flavor/engine
            vec![
                (22, 23),
                (28, 29),
                (33, 34),
                (40, 41),
                (46, 47),
                (81, 82),
                (94, 96),
                (111, 112),
                (114, 115),
                (116, 117),
                (121, 122),
                (132, 133),
                (145, 146),
                (166, 167),
                (169, 170),
                (178, 179),
                (180, 181),
                (184, 186),
                (199, 200),
                (211, 212),
                (213, 214),
                (221, 222),
                (228, 229),
                (237, 238),
                (280, 281),
                (295, 296),
                (316, 317),
                (323, 324),
                (326, 327),
                (329, 330),
                (337, 338),
                (351, 354),
                (355, 357),
                (359, 360),
                (364, 365),
                (366, 367),
                (391, 392),
                (395, 396),
                (405, 406),
                (407, 408),
                (425, 426),
                (438, 440),
            ]
        );
    }
}
