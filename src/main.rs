pub mod scoring;
pub mod tools;

use clap::Parser;
use std::ops::RangeInclusive;
use std::num::ParseIntError;

use tools::*;
use scoring::*;

type Range = RangeInclusive<usize>;

fn parse_range(s: &str) -> Result<Range, ParseIntError> {
    match s.split_once('-') {
        Some((s1, s2)) => Ok(s1.parse()? ..= s2.parse()?),
        None           => { let n = s.parse()?; Ok( n ..= n) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_range() {
        assert_eq!(parse_range("4").unwrap(), 4..=4);
        assert_eq!(parse_range("1-9").unwrap(), 1..=9);
        assert!(parse_range("abc").is_err());
        assert!(parse_range("1-").is_err());
        assert!(parse_range("-1").is_err());
    }
}

/// All values but show can be either a number or a range (XX-YY).
#[derive(Parser)]
struct C {
    #[clap(long, short, value_parser = parse_range, default_value = "100")]
    /// Value of a dollar
    limit: Range,
    #[clap(long, short, value_parser = parse_range, default_value = "1")]
    /// Denominator for fractional values, at least 2 must have
    fraction: Range,
    #[clap(long, short, default_value = "sum")]
    /// Scoring method
    scoring: ScoreType,
    #[clap(short = 'S', long, default_value_t = 5)]
    /// Max to show in case of tie (0 for all)
    show: usize,
    /// Number of denominations
    #[clap(value_parser = parse_range)]
    num: Range,
}

fn best_wrapper<E: Scoring>(scoring: E, spec: &Spec, show: usize) -> Bests {
    find_bests(&scoring, spec, enumerate(spec), show)
}

fn main() {
    let C { limit: tops, fraction: fracs, num: nums, show, scoring }
        = C::parse();
    for top in tops {
        if top == 0 { continue }
        println!("Up to {}¢", top - 1);
        for num in nums.clone() {
            if num == 0 || num >= top { continue }
            println!(" Minting {}", num);
            let mut first = 0;
            for frac in fracs.clone() {
                if frac > 1 {
                    if num == 1 || num == top - 1 { continue }
                    println!("  {} = 1¢", show_coin(frac + 1, frac));
                }
                let spec = &Spec { top, frac, num };
                let Bests { bests, score, ties } = match scoring {
                    ScoreType::Sum  => best_wrapper(Sum, spec, show),
                    ScoreType::Max  => best_wrapper(Max, spec, show),
                    ScoreType::Quad => best_wrapper(Quad, spec, show),
                };
                if frac == 1 { first = score }
                for b in bests.iter() {
                    println!("   {} => {} {}{}",
                        show_coins(spec.frac, b),
                        match scoring {
                            ScoreType::Sum  => "need",
                            ScoreType::Max  => "max",
                            ScoreType::Quad => "score",
                        },
                        score,
                        if frac > 1 && score <= first { " *" } else { "" }
                    );
                }
                if ties > show {
                    println!("    … and {} more", ties - show);
                }
            }
        }
    }
}

