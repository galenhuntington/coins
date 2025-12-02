pub mod tools;

use tools::*;
use clap::*;
use std::ops::RangeInclusive;
use std::num::ParseIntError;
use std::str::FromStr;

type Range = RangeInclusive<usize>;

fn parse_range(s: &str) -> Result<Range, ParseIntError> {
    match s.find("-") {
        Some(i) =>
            match (usize::from_str(&s[..i]), usize::from_str(&s[i+1 ..])) {
                (Ok(m), Ok(n)) => Ok(m ..= n),
                (Err(e), _) | (_, Err(e)) => Err(e),
            },
        None =>
            match usize::from_str(s) {
                Ok(n)  => Ok(n ..= n),
                Err(e) => Err(e),
            },
    }
}

/// All values but show can be either a number or a range (XX-YY).
#[derive(Debug,Parser)]
struct C {
    #[clap(long, short, value_parser = parse_range, default_value = "100")]
    /// Value of a dollar
    limit: Range,
    #[clap(long, short, value_parser = parse_range, default_value = "1")]
    /// Denominator for fractional values, at least 2 must have
    fraction: Range,
    #[clap(long, short, default_value_t = 5)]
    /// Max to show in case of tie (0 for all)
    show: usize,
    /// Number of denominations
    #[clap(value_parser = parse_range)]
    num: Range,
}

fn main() {
    let C { limit: tops, fraction: fracs, num: nums, show } = C::parse();
    for top in tops {
        if top == 0 { continue }
        println!("Up to {}¢", top - 1);
        for num in nums.clone() {
            if num == 0 || num >= top { continue }
            println!(" Minting {}", num);
            let mut first = 0;
            for frac in fracs.clone() {
                if frac > 1 && (num == 1 || num == top - 1) { continue }
                if frac > 1 { println!("  {} = 1¢", show_coin(frac + 1, frac)); }
                let sp = Spec { top, frac, num };
                let (bests, need) = find_bests(&sp, enumerate(&sp));
                if frac == 1 { first = need }
                for (i, b) in bests.iter().enumerate() {
                    if show > 0 && i >= show {
                        println!("    … and {} more", bests.len() - show);
                        break
                    }
                    println!("   {:} => need {}{}",
                        show_coins(sp.frac, b),
                        need,
                        if frac > 1 && need <= first { " *" } else { "" }
                        );
                }
            }
        }
    }
}

