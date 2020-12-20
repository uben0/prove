use super::property::Prop;

pub enum Command {
    Hypothesis,
    IntroImplication,
    IntrosImplication,
    ElimDisjonction(Prop, Prop),
    Exfalso,
    ModusPonens(Prop),
    IntroConjonction,
    ElimConjonction(Prop, Prop),
    IntroDisjonctionL,
    IntroDisjonctionR,
    Weakened(Vec<usize>),
    Apply(usize),
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
        Ok(Vec::new())
    }
}
fn indexes(buffer: &str) -> Result<Vec<usize>, &'static str> {
    if let Some(pos) = buffer.find(' ') {
        let (_, args) = buffer.split_at(pos);
        Ok(args
            .trim()
            .split(' ')
            .map(|a| a.trim().parse::<usize>())
            .collect::<Result<_, _>>()
            .map_err(|_| "not an integer")?)
    } else {
        Ok(Vec::new())
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
            "h" => match &props(s)?[..] {
                [] => Ok(Self::Hypothesis),
                _ => ea0,
            },
            "i" => match &props(s)?[..] {
                [] => Ok(Self::IntroImplication),
                _ => ea0,
            },
            "ii" => match &props(s)?[..] {
                [] => Ok(Self::IntrosImplication),
                _ => ea0,
            },
            "de" => match &props(s)?[..] {
                [a, b] => Ok(Self::ElimDisjonction(a.clone(), b.clone())),
                _ => ea2,
            },
            "f" => match &props(s)?[..] {
                [] => Ok(Self::Exfalso),
                _ => ea0,
            },
            "mp" => match &props(s)?[..] {
                [b] => Ok(Self::ModusPonens(b.clone())),
                _ => ea1,
            },
            "ci" => match &props(s)?[..] {
                [] => Ok(Self::IntroConjonction),
                _ => ea0,
            },
            "ce" => match &props(s)?[..] {
                [a, b] => Ok(Self::ElimConjonction(a.clone(), b.clone())),
                _ => ea2,
            },
            "dil" => match &props(s)?[..] {
                [] => Ok(Self::IntroDisjonctionL),
                _ => ea0,
            },
            "dir" => match &props(s)?[..] {
                [] => Ok(Self::IntroDisjonctionR),
                _ => ea0,
            },
            "af" => Ok(Self::Weakened(indexes(s)?)),
            "ap" => match &indexes(s)?[..] {
                [i] => Ok(Self::Apply(*i)),
                _ => ea1,
            },
            _ => Err("unknown command"),
        }
    }
}
