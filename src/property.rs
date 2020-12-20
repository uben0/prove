use super::sym;

/// Represents any property, eg: `A/\B->B`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prop {
    False,
    Variable(String),
    Conjonction(Box<Prop>, Box<Prop>),
    Disjonction(Box<Prop>, Box<Prop>),
    Implication(Box<Prop>, Box<Prop>),
}

impl Prop {
    /// Builds the `Variable` variant
    pub fn var(name: &str) -> Self {
        Self::Variable(name.to_owned())
    }
    /// Builds the `Conjonction` variant
    pub fn and(&self, rhs: &Self) -> Self {
        Self::Conjonction(self.clone().into(), rhs.clone().into())
    }
    /// Builds the `Disjonction` variant
    pub fn or(&self, rhs: &Self) -> Self {
        Self::Disjonction(self.clone().into(), rhs.clone().into())
    }
    /// Builds the `Implication` variant
    pub fn implies(&self, rhs: &Self) -> Self {
        Self::Implication(self.clone().into(), rhs.clone().into())
    }
    /// Builds the negation of `self`
    pub fn negate(&self) -> Self {
        self.implies(&Self::False)
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::False => Precedence::ATOMIC,
            Self::Variable(_) => Precedence::ATOMIC,
            Self::Conjonction(_, _) => Precedence::CONJONCTION,
            Self::Disjonction(_, _) => Precedence::DISJONCTION,
            Self::Implication(_, rhs) => {
                if let Self::False = rhs.as_ref() {
                    Precedence::ATOMIC
                } else {
                    Precedence::IMPLICATION
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Precedence(usize);
impl Precedence {
    const ATOMIC: Self = Self(0);
    const CONJONCTION: Self = Self(1);
    const DISJONCTION: Self = Self(2);
    const IMPLICATION: Self = Self(3);
}

use std::fmt;
impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (po, pc) = if f.alternate() {
            ("\x1b[2m(\x1b[0m", "\x1b[2m)\x1b[0m")
        } else {
            ("(", ")")
        };
        let display_binary_op = |f: &mut fmt::Formatter<'_>,
                                 lhs: &Prop,
                                 rhs: &Prop,
                                 repr: &str,
                                 preced: Precedence|
         -> fmt::Result {
            if lhs.precedence() >= preced {
                po.fmt(f)?;
                lhs.fmt(f)?;
                pc.fmt(f)?;
            } else {
                lhs.fmt(f)?;
            }
            repr.fmt(f)?;
            if rhs.precedence() > preced {
                po.fmt(f)?;
                rhs.fmt(f)?;
                pc.fmt(f)
            } else {
                rhs.fmt(f)
            }
        };
        match self {
            Self::False => sym::FALSE.fmt(f),
            Self::Variable(name) => {
                if f.alternate() {
                    "\x1b[96m".fmt(f)?;
                    name.fmt(f)?;
                    "\x1b[0m".fmt(f)
                } else {
                    name.fmt(f)
                }
            }
            Self::Conjonction(lhs, rhs) => {
                display_binary_op(f, lhs, rhs, sym::CONJONCTION, Precedence::CONJONCTION)
            }
            Self::Disjonction(lhs, rhs) => {
                display_binary_op(f, lhs, rhs, sym::DISJONCTION, Precedence::DISJONCTION)
            }
            Self::Implication(lhs, rhs) => {
                if rhs.as_ref() == &Self::False {
                    "~".fmt(f)?;
                    if lhs.precedence() > Precedence::ATOMIC {
                        po.fmt(f)?;
                        lhs.fmt(f)?;
                        pc.fmt(f)
                    } else {
                        lhs.fmt(f)
                    }
                } else {
                    display_binary_op(f, lhs, rhs, sym::IMPLICATION, Precedence::IMPLICATION)
                }
            }
        }
    }
}

mod parser {
    use super::{Precedence, Prop};
    use std::iter::Peekable;
    use std::str::FromStr;

    impl FromStr for Prop {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut lexer = Lexer::new(s);
            let iter = ParenLexer(&mut lexer);
            let v: Vec<_> = iter.collect::<Result<_, _>>().unwrap();
            syntax_parse(&v)
        }
    }

    #[derive(Debug)]
    enum LexItem {
        ParenOpen,    // (
        ParenClose,   // )
        False,        // !
        Name(String), // [A-Za-z][A-Za-z0-9_]*
        Conjonction,  // /\
        Disjonction,  // \/
        Implication,  // ->
        Negation,     // ~
    }
    struct Lexer<'a> {
        input: Peekable<std::str::Chars<'a>>,
    }
    impl<'a> Lexer<'a> {
        fn must_follow(&mut self, c: char) -> Result<(), &'static str> {
            if self.input.next().ok_or("unexpected end of stream")? == c {
                Ok(())
            } else {
                Err("unexpected character")
            }
        }
        fn new(s: &'a str) -> Self {
            Self {
                input: s.chars().peekable(),
            }
        }
    }
    impl<'a> Iterator for Lexer<'a> {
        type Item = Result<LexItem, &'static str>;
        fn next(&mut self) -> Option<Self::Item> {
            Some(match self.input.next()? {
                '(' => Ok(LexItem::ParenOpen),
                ')' => Ok(LexItem::ParenClose),
                '!' => Ok(LexItem::False),
                '~' => Ok(LexItem::Negation),
                '/' => self.must_follow('\\').map(|()| LexItem::Conjonction),
                '\\' => self.must_follow('/').map(|()| LexItem::Disjonction),
                '-' => self.must_follow('>').map(|()| LexItem::Implication),
                c @ 'A'..='Z' | c @ 'a'..='z' => {
                    let mut name = String::new();
                    name.push(c);
                    while self
                        .input
                        .peek()
                        .map(|c| matches!(c, 'A'..='Z'|'a'..='z'|'_'|'0'..='9'))
                        .unwrap_or(false)
                    {
                        name.push(self.input.next().unwrap());
                    }
                    Ok(LexItem::Name(name))
                }
                ' ' => self.next()?,
                _ => Err("unexpected character"),
            })
        }
    }

    #[derive(Debug)]
    enum ParenLexItem {
        Paren(Vec<Self>),
        False,
        Name(String),
        Conjonction,
        Disjonction,
        Implication,
        Negation,
    }
    struct ParenLexer<'a>(&'a mut dyn Iterator<Item = Result<LexItem, &'static str>>);
    impl<'a> Iterator for ParenLexer<'a> {
        type Item = Result<ParenLexItem, &'static str>;
        fn next(&mut self) -> Option<Self::Item> {
            let Self(input) = self;
            Some(match input.next()? {
                Ok(v) => Ok(match v {
                    LexItem::ParenOpen => {
                        let mut lvl = 0;
                        let mut closing = false;
                        match ParenLexer(&mut input.take_while(|v| match v {
                            Ok(LexItem::ParenOpen) => {
                                lvl += 1;
                                true
                            }
                            Ok(LexItem::ParenClose) => {
                                if lvl == 0 {
                                    closing = true;
                                    false
                                } else {
                                    lvl -= 1;
                                    true
                                }
                            }
                            _ => true,
                        }))
                        .collect::<Result<_, _>>()
                        {
                            Ok(v) => {
                                if closing {
                                    ParenLexItem::Paren(v)
                                } else {
                                    return Some(Err("closing parenthesis expected"));
                                }
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    LexItem::ParenClose => return Some(Err("unexpected closing parenthesis")),
                    LexItem::False => ParenLexItem::False,
                    LexItem::Name(name) => ParenLexItem::Name(name),
                    LexItem::Conjonction => ParenLexItem::Conjonction,
                    LexItem::Disjonction => ParenLexItem::Disjonction,
                    LexItem::Implication => ParenLexItem::Implication,
                    LexItem::Negation => ParenLexItem::Negation,
                }),
                Err(e) => Err(e),
            })
        }
    }

    fn syntax_parse(items: &[ParenLexItem]) -> Result<Prop, &'static str> {
        fn split_at_weak(
            items: &[ParenLexItem],
        ) -> Option<(&[ParenLexItem], &ParenLexItem, &[ParenLexItem])> {
            let mut max_preced = None;
            let mut index = None;
            for (i, p) in items
                .iter()
                .map(|e| match e {
                    ParenLexItem::False => None,
                    ParenLexItem::Name(_) => None,
                    ParenLexItem::Paren(_) => None,
                    ParenLexItem::Conjonction => Some(Precedence::CONJONCTION),
                    ParenLexItem::Disjonction => Some(Precedence::DISJONCTION),
                    ParenLexItem::Implication => Some(Precedence::IMPLICATION),
                    ParenLexItem::Negation => Some(Precedence::ATOMIC),
                })
                .enumerate()
            {
                if p > max_preced {
                    max_preced = p;
                    index = Some(i);
                }
            }
            let (left, right) = items.split_at(index?);
            let (op, right) = right.split_first()?;
            Some((left, op, right))
        }
        match items {
            [] => Err("empty expression"),
            [ParenLexItem::False] => Ok(Prop::False),
            [ParenLexItem::Name(name)] => Ok(Prop::Variable(name.clone())),
            [ParenLexItem::Paren(v)] => syntax_parse(v),
            // [_] => Err("not an atomic expression"),
            items => {
                if let Some((left, op, right)) = split_at_weak(items) {
                    if let ParenLexItem::Negation = op {
                        if let [] = left {
                            let right = Box::new(syntax_parse(right)?);
                            Ok(Prop::Implication(right, Prop::False.into()))
                        } else {
                            Err("negation is not a binary operator")
                        }
                    } else {
                        let (left, right) = (
                            Box::new(syntax_parse(left)?),
                            Box::new(syntax_parse(right)?),
                        );
                        Ok(match op {
                            ParenLexItem::Conjonction => Prop::Conjonction(left, right),
                            ParenLexItem::Disjonction => Prop::Disjonction(left, right),
                            ParenLexItem::Implication => Prop::Implication(left, right),
                            ParenLexItem::Negation => unreachable!(),
                            ParenLexItem::False => unreachable!(),
                            ParenLexItem::Name(_) => unreachable!(),
                            ParenLexItem::Paren(_) => unreachable!(),
                        })
                    }
                } else {
                    Err("operator not found")
                }
            }
        }
    }
}
