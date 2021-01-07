use std::io::prelude::*;

const APLHA_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const APLHA_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMER: &str = "0123456789";

trait Regex {
    fn matches(&self, regexpr: &str) -> Option<Match>;
}

#[derive(Debug)]
struct Match {
    s: String,
    groups: Vec<String>
}

impl Regex for String {
    fn matches(&self, regexpr: &str) -> Option<Match> {
        let mut str = self.chars().collect::<Vec<char>>();
        let mut regexpr = regexpr.chars().collect::<Vec<char>>();
        let mut m = Match {
            s: String::new(),
            groups: Vec::new()
        };

        if regexpr[0] == '^' {
            if regexpr[1] != str[0] {
                return None;
            }
        }

        let mut ndx: usize;
        let mut rgx = 0;
        println!("{:?}", regexpr);
        // let mut mat = false;

        loop {
            println!("{}", rgx);
            let c = regexpr[rgx];
            if c == '.' {
                ndx = 0;
            } else {
                ndx = match self.find(c) {
                    Some(i) => i,
                    None => return None
                };
            }

            while str[ndx] == regexpr[rgx] || regexpr[rgx] == '.' {
                rgx += 1;
                ndx += 1;

                if rgx == regexpr.len() {
                    m.s = self[(ndx-rgx)..ndx].to_string();
                    return Some(m);
                }
            }
            str = str[..rgx].to_vec();
            rgx = 0;
        }

        Some(m)
    }
}

fn main() {
    let s = String::from("Hello, world!");
    let m = s.matches("Helo.");
    println!("{:?}", m);
    let m = s.matches("wor");
    println!("{:?}", m);
    let m = s.matches("WORL");
    println!("{:?}", m);
}
