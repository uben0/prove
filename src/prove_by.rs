use super::property::Prop;

#[derive(Debug, Clone)]
pub enum ProveBy {
    Hypothesis,
    ImplicationIntroduction,
    Introduction,
    ModusPonens(Prop),
    DisjonctionIntroductionLeft,
    DisjonctionIntroductionRight,
    Exfalso,
    ImplicationIntroductions,
    DisjonctionElimination(Prop, Prop),
    Eliminate(usize),
    ConjonctionIntroduction,
    ConjonctionElimination(Prop, Prop),
}

fn split_once(s: &str, c: char) -> (&str, &str) {
    if let Some(i) = s.find(c) {
        let (l, r) = s.split_at(i);
        (l.trim(), r[1..].trim())
    } else {
        let (l, r) = s.split_at(s.len());
        (l.trim(), r)
    }
}

fn expect_end(s: &str) -> Result<(), &'static str> {
    if s.trim().is_empty() {
        Ok(())
    } else {
        Err("unexpected argument")
    }
}
fn parse_arg_prop(s: &str) -> Result<(Prop, &str), &'static str> {
    let (l, r) = split_once(s, ',');
    Ok((l.parse()?, r))
}
fn parse_arg_num(s: &str) -> Result<(usize, &str), &'static str> {
    let (l, r) = split_once(s, ' ');
    Ok((l.parse().map_err(|_| "not a number")?, r))
}

impl std::str::FromStr for ProveBy {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, args) = split_once(s, ' ');
        match name {
            "h" => {expect_end(args)?; Ok(Self::Hypothesis)}
            "ii" => {expect_end(args)?; Ok(Self::ImplicationIntroduction)}
            "ci" => {expect_end(args)?; Ok(Self::ConjonctionIntroduction)}
            "iis" => {expect_end(args)?; Ok(Self::ImplicationIntroductions)}
            "i" => {expect_end(args)?; Ok(Self::Introduction)}
            "mp" => {
                let (arg1, args) = parse_arg_prop(args)?;
                expect_end(args)?;
                Ok(Self::ModusPonens(arg1))
            }
            "e" => {
                let (arg1, args) = parse_arg_num(args)?;
                expect_end(args)?;
                Ok(Self::Eliminate(arg1))
            }
            "dil" => {
                expect_end(args)?;
                Ok(Self::DisjonctionIntroductionLeft)
            }
            "dir" => {
                expect_end(args)?;
                Ok(Self::DisjonctionIntroductionRight)
            }
            "de" => {
                let (arg1, args) = parse_arg_prop(args)?;
                let (arg2, args) = parse_arg_prop(args)?;
                expect_end(args)?;
                Ok(Self::DisjonctionElimination(arg1, arg2))
            }
            "ce" => {
                let (arg1, args) = parse_arg_prop(args)?;
                let (arg2, args) = parse_arg_prop(args)?;
                expect_end(args)?;
                Ok(Self::ConjonctionElimination(arg1, arg2))
            }
            "xf" => {
                expect_end(args)?;
                Ok(Self::Exfalso)
            }
            _ => Err("unknown rule application"),
        }
    }
}