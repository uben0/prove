use super::property::Prop;

pub enum Command {
    Hypothesis,                  // hyp
    IntroImplication,            // impl_i
    ElimDisjonction(Prop, Prop), // disj_e A, B
    Exfalso,                     // false
    ModusPonens(Prop),           // mp B
    IntroConjonction,            // conj_i
    ElimConjonction(Prop, Prop), // conj_e A, B
    IntroDisjonctionL,           // disj_i_l
    IntroDisjonctionR,           // disj_i_r
    Weakened(Vec<usize>),        // aff E
}

fn parse_name(buffer: &str) -> Result<&str, &'static str> {
    if let Some(pos) = buffer.find(' ') {
        let (name, _) = buffer.split_at(pos);
        Ok(name)
    } else {
        Ok(buffer)
    }
}

fn props(buffer: &str) -> Result<Vec<Prop>, &'static str> {
    if let Some(pos) = buffer.find(' ') {
        let (_, args) = buffer.split_at(pos);
        Ok(args
            .split(',')
            .map(|a| a.parse())
            .collect::<Result<_, _>>()?)
    } else {
        Err("no args")
    }
}
fn indexes(buffer: &str) -> Result<Vec<usize>, &'static str> {
    if let Some(pos) = buffer.find(' ') {
        let (_, args) = buffer.split_at(pos);
        Ok(args
            .split(',')
            .map(|a| a.trim().parse::<usize>())
            .collect::<Result<_, _>>()
            .map_err(|_| "not an integer")?)
    } else {
        Err("no args")
    }
}

use std::str::FromStr;
impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ea0 = Err("takes 0 arguments");
        let ea1 = Err("takes 1 arguments");
        let ea2 = Err("takes 2 arguments");
        match parse_name(s)? {
            "hyp" => match &props(s)?[..] {
                [] => Ok(Self::Hypothesis),
                _ => ea0,
            },
            "impl_i" => match &props(s)?[..] {
                [] => Ok(Self::IntroImplication),
                _ => ea0,
            },
            "disj_e" => match &props(s)?[..] {
                [a, b] => Ok(Self::ElimDisjonction(a.clone(), b.clone())),
                _ => ea2,
            },
            "false" => match &props(s)?[..] {
                [] => Ok(Self::Exfalso),
                _ => ea0,
            },
            "mp" => match &props(s)?[..] {
                [b] => Ok(Self::ModusPonens(b.clone())),
                _ => ea1,
            },
            "conj_i" => match &props(s)?[..] {
                [] => Ok(Self::IntroConjonction),
                _ => ea0,
            },
            "conj_e" => match &props(s)?[..] {
                [a, b] => Ok(Self::ElimConjonction(a.clone(), b.clone())),
                _ => ea2,
            },
            "disj_i_l" => match &props(s)?[..] {
                [] => Ok(Self::IntroDisjonctionL),
                _ => ea0,
            },
            "disj_i_r" => match &props(s)?[..] {
                [] => Ok(Self::IntroDisjonctionR),
                _ => ea0,
            },
            "aff" => Ok(Self::Weakened(indexes(s)?)),
            _ => Err("unknown command"),
        }
    }
}
