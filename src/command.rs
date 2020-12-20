use super::property::Prop;

pub enum Command {
    Hypothesis,                    // hyp
    IntroImplication,              // impl_i
    ElimDisjonction(Prop, Prop),   // disj_e A, B
    Exfalso,                       // false
}

fn parse_with_model(buffer: &str) -> Result<(&str, Vec<Prop>), &'static str> {
    if let Some(pos) = buffer.find(' ') {
        let (name, args) = buffer.split_at(pos);
        Ok((name, args.split(',').map(|a| a.parse()).collect::<Result<_,_>>()?))
    }
    else {
        Ok((buffer, Vec::new()))
    }
}

use std::str::FromStr;
impl FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, args) = parse_with_model(s)?;
        match (name, &args[..]) {
            ("hyp", []) => Ok(Self::Hypothesis),
            ("impl_i", []) => Ok(Self::IntroImplication),
            ("disj_e", [a, b]) => Ok(Self::ElimDisjonction(a.clone(), b.clone())),
            ("false", []) => Ok(Self::Exfalso),
            _ => Err("unknown command"),
        }
    }
}