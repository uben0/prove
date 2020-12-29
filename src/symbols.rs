#[derive(Default, Clone, Copy)]
pub struct ReprConf {
    pub negation: bool,
    pub formated: bool,
    pub unicode: bool,
    pub emphazis: bool,
}

use std::fmt;

#[derive(Clone, Copy)]
pub enum Sym {
    OpenParenthesis,
    CloseParenthesis,
    False,
    Conjonction,
    Disjonction,
    Negation,
    Implication,
    Equivalence,
    Sequent,
    Comma,
    RuleLine,
    RuleHypothesis,
    RuleImplicationIntroduction,
    RuleImplicationIntroductions,
    RuleModusPonens,
    RuleWeakModusPonens,
    RuleDisjonctionIntroductionLeft,
    RuleDisjonctionIntroductionRight,
    RuleExfalso,
    RuleDisjonctionElimination,
    RuleDisjonctionWeakElimination,
    RuleEquivalenceIntroduction,
    RuleEquivalenceElimination,
    RuleEquivalenceWeakElimination,
    RuleConjonctionIntroduction,
    RuleConjonctionElimination,
    RuleConjonctionWeakElimination,
}

impl Sym {
    pub const fn lex(&self) -> &str {
        match self {
            Self::OpenParenthesis => "(",
            Self::CloseParenthesis => ")",
            Self::Conjonction => "/\\",
            Self::Disjonction => "\\/",
            Self::False => "!",
            Self::Negation => "~",
            Self::Implication => "->",
            Self::Equivalence => "<->",
            Self::Sequent => "|-",
            Self::Comma => ",",
            Self::RuleLine => "─",
            Self::RuleHypothesis => "h",
            Self::RuleImplicationIntroduction => "->i",
            Self::RuleImplicationIntroductions => "->i'",
            Self::RuleModusPonens => "mp",
            Self::RuleWeakModusPonens => "mp'",
            Self::RuleDisjonctionIntroductionLeft => "\\/i,l",
            Self::RuleDisjonctionIntroductionRight => "\\/i,r",
            Self::RuleDisjonctionElimination => "\\/e",
            Self::RuleDisjonctionWeakElimination => "\\/e'",
            Self::RuleExfalso => "!e",
            Self::RuleEquivalenceIntroduction => "<->i",
            Self::RuleEquivalenceElimination => "<->e",
            Self::RuleEquivalenceWeakElimination => "<->e'",
            Self::RuleConjonctionIntroduction => "/\\i",
            Self::RuleConjonctionElimination => "/\\e",
            Self::RuleConjonctionWeakElimination => "/\\e'",
        }
    }
    pub fn repr(&self, unicode: bool) -> &str {
        match (unicode, self) {
            (true, Self::False) => "⊥",
            (true, Self::Conjonction) => "∧",
            (true, Self::Disjonction) => "∨",
            (true, Self::Implication) => "➔",
            (true, Self::Sequent) => "⊢",
            (true, Self::RuleImplicationIntroduction) => "➔i",
            _ => self.lex(),
        }
    }
    pub fn len(&self, unicode: bool) -> usize {
        self.repr(unicode).chars().count()
    }
    fn color(&self, formated: bool) -> Option<(&str, &str)> {
        if formated {
            match self {
                Self::OpenParenthesis | Self::CloseParenthesis => Some(("\x1b[2m", "\x1b[0m")),
                Self::Sequent | Self::Comma => Some(("\x1b[1m", "\x1b[0m")),
                _ => None,
            }
        } else {
            None
        }
    }
    pub fn fmt(&self, conf: ReprConf, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Display;
        if let Some((color_set, color_unset)) = self.color(conf.formated) {
            color_set.fmt(f)?;
            self.repr(conf.unicode).fmt(f)?;
            color_unset.fmt(f)
        } else {
            self.repr(conf.unicode).fmt(f)
        }
    }
}

pub struct Paren<T>(pub T, pub ReprConf);
impl<T: fmt::Display> fmt::Display for Paren<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Sym::OpenParenthesis.fmt(self.1, f)?;
        self.0.fmt(f)?;
        Sym::CloseParenthesis.fmt(self.1, f)
    }
}