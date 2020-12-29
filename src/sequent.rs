use super::property::Prop;
use super::symbols;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sequent {
    hypotheses: Vec<Prop>,
    conclusion: Prop,
}
impl Sequent {
    pub fn new(hypotheses: Vec<Prop>, conclusion: Prop) -> Self {
        Self {
            hypotheses,
            conclusion,
        }
    }
    pub fn repr(&self) -> SequentRepr {
        self.repr_conf(Default::default())
    }
    pub fn repr_conf(&self, conf: symbols::ReprConf) -> SequentRepr {
        SequentRepr {
            sequent: self,
            conf,
        }
    }
    pub fn hypotheses(&self) -> &[Prop] {
        &self.hypotheses
    }
    pub fn conclusion(&self) -> &Prop {
        &self.conclusion
    }
}

#[derive(Clone)]
pub struct SequentRepr<'a> {
    sequent: &'a Sequent,
    conf: symbols::ReprConf,
}
impl<'a> SequentRepr<'a> {
    pub fn formated(mut self) -> Self {
        self.conf.formated = true;
        self
    }
    pub fn unicode(mut self) -> Self {
        self.conf.unicode = true;
        self
    }
    pub fn negation(mut self) -> Self {
        self.conf.negation = true;
        self
    }
    pub fn len(&self) -> usize {
        let mut repr = self.clone();
        repr.conf.formated = false;
        repr.to_string().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl<'a> fmt::Display for SequentRepr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for h in &self.sequent.hypotheses {
            if !first {
                symbols::Sym::Comma.fmt(self.conf, f)?;
                " ".fmt(f)?;
            }
            first = false;
            h.repr_conf(self.conf).fmt(f)?;
        }
        if !self.sequent.hypotheses.is_empty() {
            " ".fmt(f)?;
        }
        symbols::Sym::Sequent.fmt(self.conf, f)?;
        " ".fmt(f)?;
        let mut emph_conf = self.conf;
        emph_conf.emphazis = true;
        self.sequent.conclusion.repr_conf(emph_conf).fmt(f)
    }
}

impl std::str::FromStr for Sequent {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(symbols::Sym::Sequent.lex());
        match [iter.next(), iter.next(), iter.next()] {
            [Some(left), Some(right), None] => {
                let hypotheses: Vec<_> = if left.trim().is_empty() {
                    Vec::new()
                } else {
                    left.split(symbols::Sym::Comma.lex())
                        .map(|h| h.parse())
                        .collect::<Result<_, _>>()?
                };
                Ok(Self::new(hypotheses, right.parse()?))
            }
            [Some(prop), None, None] => Ok(Self::new(Vec::new(), prop.parse()?)),
            _ => Err("expecting only one sequent symbol"),
        }
    }
}