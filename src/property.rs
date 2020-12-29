use super::symbols;
use std::fmt;

/// Represents any property, eg: `A/\B->B`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Prop {
    False,
    Variable(String),
    Conjonction(Box<Prop>, Box<Prop>),
    Disjonction(Box<Prop>, Box<Prop>),
    Implication(Box<Prop>, Box<Prop>),
    Equivalence(Box<Prop>, Box<Prop>),
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
    /// Wraps a property to control the way it will be displayed
    pub fn repr(&self) -> PropRepr {
        self.repr_conf(Default::default())
    }
    pub fn repr_conf(&self, conf: symbols::ReprConf) -> PropRepr {
        PropRepr { prop: self, conf }
    }
    fn precedence(&self, negation: bool) -> Precedence {
        match self {
            Self::False => Precedence::ATOMIC,
            Self::Variable(_) => Precedence::ATOMIC,
            Self::Conjonction(_, _) => Precedence::CONJONCTION,
            Self::Disjonction(_, _) => Precedence::DISJONCTION,
            Self::Implication(_, lhs) => {
                if negation && lhs.as_ref() == &Self::False {
                    Precedence::NEGATION
                } else {
                    Precedence::IMPLICATION
                }
            }
            Self::Equivalence(_, _) => Precedence::EQUIVALENCE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Precedence(usize);
impl Precedence {
    const ATOMIC: Self = Self(0);
    const NEGATION: Self = Self(1);
    const CONJONCTION: Self = Self(2);
    const DISJONCTION: Self = Self(3);
    const IMPLICATION: Self = Self(4);
    const EQUIVALENCE: Self = Self(5);
}

#[derive(Clone)]
pub struct PropRepr<'a> {
    prop: &'a Prop,
    conf: symbols::ReprConf,
}
impl<'a> PropRepr<'a> {
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
    fn precedence(&self) -> Precedence {
        self.prop.precedence(self.conf.negation)
    }
}
impl<'a> fmt::Display for PropRepr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn bin_op(
            f: &mut fmt::Formatter<'_>,
            lhs: &Prop,
            rhs: &Prop,
            conf: symbols::ReprConf,
            preced: Precedence,
            sym: symbols::Sym,
        ) -> fmt::Result {
            let (lhs, rhs) = (PropRepr { prop: lhs, conf }, PropRepr { prop: rhs, conf });
            if lhs.precedence() >= preced {
                symbols::Paren(lhs, conf).fmt(f)?;
            } else {
                lhs.fmt(f)?;
            }
            sym.fmt(conf, f)?;
            if rhs.precedence() > preced {
                symbols::Paren(rhs, conf).fmt(f)
            } else {
                rhs.fmt(f)
            }
        }
        match self.prop {
            Prop::False => symbols::Sym::False.fmt(self.conf, f),
            Prop::Variable(name) => {
                if self.conf.formated {
                    if self.conf.emphazis {
                        "\x1b[96m"
                    } else {
                        "\x1b[92m"
                    }.fmt(f)?;
                }
                name.fmt(f)?;
                if self.conf.formated {
                    "\x1b[0m".fmt(f)?
                }
                Ok(())
            }
            Prop::Conjonction(lhs, rhs) => bin_op(
                f,
                lhs,
                rhs,
                self.conf,
                self.precedence(),
                symbols::Sym::Conjonction,
            ),
            Prop::Disjonction(lhs, rhs) => bin_op(
                f,
                lhs,
                rhs,
                self.conf,
                self.precedence(),
                symbols::Sym::Disjonction,
            ),
            Prop::Implication(lhs, rhs) => {
                if self.conf.negation && rhs.as_ref() == &Prop::False {
                    let lhs = PropRepr {
                        prop: lhs,
                        conf: self.conf,
                    };
                    symbols::Sym::Negation.fmt(self.conf, f)?;
                    if lhs.precedence() > self.precedence() {
                        symbols::Paren(lhs, self.conf).fmt(f)
                    } else {
                        lhs.fmt(f)
                    }
                } else {
                    bin_op(
                        f,
                        lhs,
                        rhs,
                        self.conf,
                        self.precedence(),
                        symbols::Sym::Implication,
                    )
                }
            }
            Prop::Equivalence(lhs, rhs) => bin_op(
                f,
                lhs,
                rhs,
                self.conf,
                self.precedence(),
                symbols::Sym::Equivalence,
            ),
        }
    }
}

mod parser {
    use super::Prop;

    impl std::str::FromStr for Prop {
        type Err = &'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let v: Vec<_> = Lexer::new(&mut s.chars()).collect::<Result<_, _>>()?;
            syntax_parse(&v)
        }
    }

    #[derive(Debug)]
    enum LexItem {
        Parenthesized(Vec<Self>), // ( .* )
        False,                    // !
        Name(String),             // [A-Za-z][A-Za-z0-9_]*
        Conjonction,              // /\
        Disjonction,              // \/
        Implication,              // ->
        Equivalence,              // <->
        Negation,                 // ~
    }

    struct Parenthesized<'a> {
        input: &'a mut dyn Iterator<Item = char>,
        level: usize,
    }
    impl<'a> Parenthesized<'a> {
        fn new(input: &'a mut impl Iterator<Item = char>) -> Self {
            Self { input, level: 0 }
        }
    }
    impl<'a> Iterator for Parenthesized<'a> {
        type Item = Result<char, ()>;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(c) = self.input.next() {
                match c {
                    '(' => {
                        self.level += 1;
                        Some(Ok(c))
                    }
                    ')' if self.level == 0 => None,
                    ')' => {
                        self.level -= 1;
                        Some(Ok(c))
                    }
                    c => Some(Ok(c)),
                }
            } else {
                Some(Err(()))
            }
        }
    }
    struct Lexer<'a> {
        input: std::iter::Peekable<&'a mut dyn Iterator<Item = char>>,
    }
    impl<'a> Lexer<'a> {
        fn new(input: &'a mut impl Iterator<Item = char>) -> Self {
            Self {
                input: (input as &mut dyn Iterator<Item = char>).peekable(),
            }
        }
        fn must_follow(&mut self, c: char) -> Result<(), &'static str> {
            if self.input.next().ok_or("unexpected end of stream")? == c {
                Ok(())
            } else {
                Err("unexpected character")
            }
        }
    }
    impl<'a> Iterator for Lexer<'a> {
        type Item = Result<LexItem, &'static str>;
        fn next(&mut self) -> Option<Self::Item> {
            Some(match self.input.next()? {
                '(' => {
                    let r: Result<Vec<char>, ()> = Parenthesized::new(&mut self.input).collect();
                    match r {
                        Err(()) => return Some(Err("closing parenthesis expected")),
                        Ok(mut v) => {
                            let r: Result<Vec<_>, _> = Lexer::new(&mut v.drain(..)).collect();
                            match r {
                                Err(e) => Err(e),
                                Ok(v) => Ok(LexItem::Parenthesized(v)),
                            }
                        }
                    }
                }
                ')' => Err("unexpected closing parenthesis"),
                '!' => Ok(LexItem::False),
                '~' => Ok(LexItem::Negation),
                '/' => self.must_follow('\\').map(|()| LexItem::Conjonction),
                '\\' => self.must_follow('/').map(|()| LexItem::Disjonction),
                '-' => self.must_follow('>').map(|()| LexItem::Implication),
                '<' => self.must_follow('-').and(self.must_follow('>')).map(|()| LexItem::Equivalence),
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
    fn syntax_parse(items: &[LexItem]) -> Result<Prop, &'static str> {
        fn split_at_weak(items: &[LexItem]) -> Option<(&[LexItem], &LexItem, &[LexItem])> {
            use super::Precedence;
            let mut max_preced = None;
            let mut index = None;
            for (i, p) in items
                .iter()
                .map(|e| match e {
                    LexItem::Conjonction => Some(Precedence::CONJONCTION),
                    LexItem::Disjonction => Some(Precedence::DISJONCTION),
                    LexItem::Implication => Some(Precedence::IMPLICATION),
                    LexItem::Equivalence => Some(Precedence::EQUIVALENCE),
                    LexItem::Negation => Some(Precedence::NEGATION),
                    _ => None,
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
            [LexItem::False] => Ok(Prop::False),
            [LexItem::Name(name)] => Ok(Prop::Variable(name.clone())),
            [LexItem::Parenthesized(v)] => syntax_parse(v),
            items => {
                if let Some((left, op, right)) = split_at_weak(items) {
                    if let LexItem::Negation = op {
                        if let [] = left {
                            Ok(syntax_parse(right)?.negate())
                        } else {
                            Err("negation is not a binary operator")
                        }
                    } else {
                        let (left, right) = (
                            Box::new(syntax_parse(left)?),
                            Box::new(syntax_parse(right)?),
                        );
                        Ok(match op {
                            LexItem::Conjonction => Prop::Conjonction(left, right),
                            LexItem::Disjonction => Prop::Disjonction(left, right),
                            LexItem::Implication => Prop::Implication(left, right),
                            LexItem::Equivalence => Prop::Equivalence(left, right),
                            _ => unreachable!(),
                        })
                    }
                } else {
                    Err("operator not found")
                }
            }
        }
    }
}
