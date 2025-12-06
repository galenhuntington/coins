pub type Score = usize;
pub const MIN_SCORE: Score = 0;

#[derive(PartialEq,Eq,Clone,Copy,clap::ValueEnum)]
pub enum ScoreType { Sum, Max, Quad }

pub trait Scoring {
    fn fold_fn(&self, a: Score, b: Score) -> Score;
}

pub struct Sum;
pub struct Max;
pub struct Quad;

impl Scoring for Sum {
    #[inline] fn fold_fn(&self, a: Score, b: usize) -> Score { a + b }
}

impl Scoring for Max {
    #[inline] fn fold_fn(&self, a: Score, b: usize) -> Score { usize::max(a, b) }
}

impl Scoring for Quad {
    #[inline] fn fold_fn(&self, a: Score, b: usize) -> Score { a + b*b }
}

