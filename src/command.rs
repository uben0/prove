use super::Sequent;

pub enum Command {
    Hypothesis,        // hyp
    IntroImplication,  // impl_i
}
impl Command {
    pub fn apply_on(&self, sequent: &mut Sequent) -> bool {
        match self {
            Self::Hypothesis => sequent.prove_by_hyp(),
            Self::IntroImplication => sequent.prove_by_impl_intro(),
        }
    }
}

use std::str::FromStr;
impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{:?}", s);
        match s {
            "hyp" => Ok(Self::Hypothesis),
            "impl_i" => Ok(Self::IntroImplication),
            _ => Err("unknown command"),
        }
    }
}