use std::fmt;

use super::property::Prop;
use super::prove_by::ProveBy;
use super::sequent::Sequent;
use super::symbols;

#[derive(Debug, Clone)]
pub enum Rule {
    Hypothesis([Proof; 0]),
    ImplicationIntroduction([Proof; 1]),
    ImplicationIntroductions([Proof; 1]),
    ModusPonens([Proof; 2]),
    WeakModusPonens([Proof; 1]),
    DisjonctionIntroductionLeft([Proof; 1]),
    DisjonctionIntroductionRight([Proof; 1]),
    DisjonctionElimination([Proof; 3]),
    DisjonctionWeakElimination([Proof; 2]),
    Exfalso([Proof; 1]),
    EquivalenceIntroduction([Proof; 2]),
    // EquivalenceElimination([Proof; 2]),
    EquivalenceWeakElimination([Proof; 1]),
    ConjonctionIntroduction([Proof; 2]),
    ConjonctionElimination([Proof; 2]),
    ConjonctionWeakElimination([Proof; 1]),
}
impl Rule {
    fn hypotheses(&self) -> &[Proof] {
        match self {
            Self::Hypothesis(v) => v,
            Self::ImplicationIntroduction(v) => v,
            Self::ImplicationIntroductions(v) => v,
            Self::ModusPonens(v) => v,
            Self::WeakModusPonens(v) => v,
            Self::DisjonctionIntroductionLeft(v) => v,
            Self::DisjonctionIntroductionRight(v) => v,
            Self::DisjonctionElimination(v) => v,
            Self::DisjonctionWeakElimination(v) => v,
            Self::Exfalso(v) => v,
            Self::EquivalenceIntroduction(v) => v,
            // Self::EquivalenceElimination(v) => v,
            Self::EquivalenceWeakElimination(v) => v,
            Self::ConjonctionIntroduction(v) => v,
            Self::ConjonctionElimination(v) => v,
            Self::ConjonctionWeakElimination(v) => v,
        }
    }
    fn hypotheses_mut(&mut self) -> &mut [Proof] {
        match self {
            Self::Hypothesis(v) => v,
            Self::ImplicationIntroduction(v) => v,
            Self::ImplicationIntroductions(v) => v,
            Self::ModusPonens(v) => v,
            Self::WeakModusPonens(v) => v,
            Self::DisjonctionIntroductionLeft(v) => v,
            Self::DisjonctionIntroductionRight(v) => v,
            Self::DisjonctionElimination(v) => v,
            Self::DisjonctionWeakElimination(v) => v,
            Self::Exfalso(v) => v,
            Self::EquivalenceIntroduction(v) => v,
            // Self::EquivalenceElimination(v) => v,
            Self::EquivalenceWeakElimination(v) => v,
            Self::ConjonctionIntroduction(v) => v,
            Self::ConjonctionElimination(v) => v,
            Self::ConjonctionWeakElimination(v) => v,
        }
    }
    fn get_symbol(&self) -> symbols::Sym {
        match self {
            Self::Hypothesis(_) => symbols::Sym::RuleHypothesis,
            Self::ImplicationIntroduction(_) => symbols::Sym::RuleImplicationIntroduction,
            Self::ImplicationIntroductions(_) => symbols::Sym::RuleImplicationIntroductions,
            Self::ModusPonens(_) => symbols::Sym::RuleModusPonens,
            Self::WeakModusPonens(_) => symbols::Sym::RuleWeakModusPonens,
            Self::DisjonctionIntroductionLeft(_) => symbols::Sym::RuleDisjonctionIntroductionLeft,
            Self::DisjonctionIntroductionRight(_) => symbols::Sym::RuleDisjonctionIntroductionRight,
            Self::DisjonctionElimination(_) => symbols::Sym::RuleDisjonctionElimination,
            Self::DisjonctionWeakElimination(_) => symbols::Sym::RuleDisjonctionWeakElimination,
            Self::Exfalso(_) => symbols::Sym::RuleExfalso,
            Self::EquivalenceIntroduction(_) => symbols::Sym::RuleEquivalenceIntroduction,
            // Self::EquivalenceElimination(_) => symbols::Sym::RuleEquivalenceElimination,
            Self::EquivalenceWeakElimination(_) => symbols::Sym::RuleEquivalenceWeakElimination,
            Self::ConjonctionIntroduction(_) => symbols::Sym::RuleConjonctionIntroduction,
            Self::ConjonctionElimination(_) => symbols::Sym::RuleConjonctionElimination,
            Self::ConjonctionWeakElimination(_) => symbols::Sym::RuleConjonctionWeakElimination,
        }
    }
    fn from(s: &Sequent, p: ProveBy) -> Option<Self> {
        match p {
            ProveBy::Hypothesis => {
                if s.hypotheses().contains(&s.conclusion()) {
                    Some(Self::Hypothesis([]))
                } else {
                    None
                }
            }
            ProveBy::ImplicationIntroduction => match s.conclusion() {
                Prop::Implication(lhs, rhs) => {
                    Some(Self::ImplicationIntroduction([Proof::not_proven(
                        Sequent::new(
                            s.hypotheses()
                                .iter()
                                .chain(std::iter::once(lhs.as_ref()))
                                .cloned()
                                .collect::<Vec<Prop>>(),
                            rhs.as_ref().clone(),
                        ),
                    )]))
                }
                _ => None,
            },
            ProveBy::ImplicationIntroductions => match s.conclusion() {
                Prop::Implication(lhs, rhs) => Some(Self::ImplicationIntroductions([{
                    let mut p = Proof::not_proven(Sequent::new(
                        s.hypotheses()
                            .iter()
                            .chain(std::iter::once(lhs.as_ref()))
                            .cloned()
                            .collect::<Vec<Prop>>(),
                        rhs.as_ref().clone(),
                    ));
                    p.prove_by(ProveBy::ImplicationIntroductions);
                    p.next_not_proven().unwrap().clone()
                }])),
                _ => None,
            },
            ProveBy::ModusPonens(prop) => Some(Self::ModusPonens([
                Proof::not_proven(Sequent::new(
                    s.hypotheses().to_owned(),
                    prop.implies(s.conclusion()),
                )),
                Proof::not_proven(Sequent::new(s.hypotheses().to_owned(), prop)),
            ])),
            ProveBy::DisjonctionIntroductionLeft => match s.conclusion() {
                Prop::Disjonction(lhs, _) => {
                    Some(Self::DisjonctionIntroductionLeft([Proof::not_proven(
                        Sequent::new(s.hypotheses().to_owned(), lhs.as_ref().clone()),
                    )]))
                }
                _ => None,
            },
            ProveBy::DisjonctionIntroductionRight => match s.conclusion() {
                Prop::Disjonction(_, rhs) => {
                    Some(Self::DisjonctionIntroductionRight([Proof::not_proven(
                        Sequent::new(s.hypotheses().to_owned(), rhs.as_ref().clone()),
                    )]))
                }
                _ => None,
            },
            ProveBy::Exfalso => Some(Self::Exfalso([Proof::not_proven(Sequent::new(
                s.hypotheses().to_owned(),
                Prop::False,
            ))])),
            ProveBy::DisjonctionElimination(a, b) => Some(Self::DisjonctionElimination([
                Proof::not_proven(Sequent::new(s.hypotheses().to_owned(), a.or(&b))),
                Proof::not_proven(Sequent::new(
                    s.hypotheses()
                        .iter()
                        .cloned()
                        .chain(std::iter::once(a))
                        .collect(),
                    s.conclusion().clone(),
                )),
                Proof::not_proven(Sequent::new(
                    s.hypotheses()
                        .iter()
                        .cloned()
                        .chain(std::iter::once(b))
                        .collect(),
                    s.conclusion().clone(),
                )),
            ])),
            ProveBy::Introduction => match s.conclusion() {
                Prop::Equivalence(lhs, rhs) => Some(Self::EquivalenceIntroduction([
                    Proof::not_proven(Sequent::new(s.hypotheses().to_owned(), lhs.implies(rhs))),
                    Proof::not_proven(Sequent::new(s.hypotheses().to_owned(), rhs.implies(lhs))),
                ])),
                Prop::Conjonction(_, _) => Self::from(s, ProveBy::ConjonctionIntroduction),
                Prop::Implication(_, _) => Self::from(s, ProveBy::ImplicationIntroduction),
                _ => None,
            },
            ProveBy::Eliminate(index) => s
                .hypotheses()
                .get(index)
                .map(|p| match p {
                    Prop::Equivalence(lhs, rhs) => {
                        Some(Self::EquivalenceWeakElimination([Proof::not_proven(
                            Sequent::new(
                                {
                                    let mut h = s.hypotheses().to_owned();
                                    h[index] = lhs.implies(rhs);
                                    h.insert(index + 1, rhs.implies(lhs));
                                    h
                                },
                                s.conclusion().clone(),
                            ),
                        )]))
                    }
                    Prop::Conjonction(lhs, rhs) => {
                        Some(Self::ConjonctionWeakElimination([Proof::not_proven(
                            Sequent::new(
                                {
                                    let mut h = s.hypotheses().to_owned();
                                    h[index] = lhs.as_ref().clone();
                                    h.insert(index + 1, rhs.as_ref().clone());
                                    h
                                },
                                s.conclusion().clone(),
                            ),
                        )]))
                    }
                    Prop::Implication(lhs, rhs) => {
                        if rhs.as_ref() == s.conclusion() {
                            Some(Self::WeakModusPonens([Proof::not_proven(Sequent::new(
                                s.hypotheses().to_owned(),
                                lhs.as_ref().clone(),
                            ))]))
                        } else {
                            None
                        }
                    }
                    Prop::Disjonction(lhs, rhs) => Some(Self::DisjonctionWeakElimination([
                        Proof::not_proven(Sequent::new(
                            {
                                let mut h = s.hypotheses().to_owned();
                                h[index] = lhs.as_ref().clone();
                                h
                            },
                            s.conclusion().clone(),
                        )),
                        Proof::not_proven(Sequent::new(
                            {
                                let mut h = s.hypotheses().to_owned();
                                h[index] = rhs.as_ref().clone();
                                h
                            },
                            s.conclusion().clone(),
                        )),
                    ])),
                    _ => None,
                })
                .flatten(),
            ProveBy::ConjonctionElimination(a, b) => Some(Self::ConjonctionElimination([
                Proof::not_proven(Sequent::new(s.hypotheses().to_owned(), a.and(&b))),
                Proof::not_proven(Sequent::new(
                    s.hypotheses()
                        .iter()
                        .cloned()
                        .chain(std::iter::once(a))
                        .chain(std::iter::once(b))
                        .collect(),
                    s.conclusion().clone(),
                )),
            ])),
            ProveBy::ConjonctionIntroduction => match s.conclusion() {
                Prop::Conjonction(lhs, rhs) => Some(Self::ConjonctionIntroduction([
                    Proof::not_proven(Sequent::new(
                        s.hypotheses().to_owned(),
                        lhs.as_ref().clone(),
                    )),
                    Proof::not_proven(Sequent::new(
                        s.hypotheses().to_owned(),
                        rhs.as_ref().clone(),
                    )),
                ])),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Proof {
    sequent: Sequent,
    rule: Option<Box<Rule>>,
}
impl Proof {
    pub fn is_complete(&self) -> bool {
        self.rule
            .as_ref()
            .map(|v| v.hypotheses().iter().all(|v| v.is_complete()))
            .unwrap_or(false)
    }
    pub fn repr_conf(&self, conf: symbols::ReprConf) -> ProofRepr {
        ProofRepr { proof: self, conf }
    }
    pub fn not_proven(s: Sequent) -> Self {
        Self {
            sequent: s,
            rule: None,
        }
    }
    pub fn new(s: Sequent, p: ProveBy) -> Self {
        match p {
            ProveBy::Hypothesis => {
                s.hypotheses().contains(&s.conclusion());
                unimplemented!()
            }
            _ => unimplemented!(),
        }
    }
    pub fn prove_by(&mut self, p: ProveBy) -> bool {
        if let Some(r) = Rule::from(&self.sequent, p) {
            self.rule = Some(r.into());
            true
        } else {
            false
        }
    }
    pub fn next_not_proven(&self) -> Option<&Self> {
        if let Some(rule) = self.rule.as_ref() {
            rule.hypotheses()
                .iter()
                .filter_map(|p| p.next_not_proven())
                .next()
        } else {
            Some(self)
        }
    }
    pub fn next_not_proven_mut(&mut self) -> Option<&mut Self> {
        if self.rule.is_some() {
            self.rule
                .as_mut()
                .unwrap()
                .as_mut()
                .hypotheses_mut()
                .iter_mut()
                .filter_map(|p| p.next_not_proven_mut())
                .next()
        } else {
            Some(self)
        }
    }
    pub fn prove_next_by(&mut self, p: ProveBy) -> bool {
        if let Some(n) = self.next_not_proven_mut() {
            n.prove_by(p)
        } else {
            false
        }
    }
}

pub struct ProofRepr<'a> {
    proof: &'a Proof,
    conf: symbols::ReprConf,
}
impl<'a> ProofRepr<'a> {
    fn render(&self) -> ProofRender {
        if let Some(rule) = self.proof.rule.as_ref() {
            let mut r = ProofRender::from_iter(
                rule.hypotheses()
                    .iter()
                    .map(|v| v.repr_conf(self.conf).render()),
                4,
            );
            let s_width = self.proof.sequent.repr_conf(self.conf).len();
            
            let dr_center = 2 * r.bottom_x + r.bottom_width;
            let ds_center = s_width;
            let dcenter = dr_center.max(ds_center);
            let r_x = (dcenter - dr_center) / 2;
            let s_x = (dcenter - ds_center) / 2;

            // // let r_center = (r.bottom_x + r.bottom_width) / 2;
            // // let s_center = s_width / 2;
            // // let center = r_center.max(s_center);
            // let r_x = center - r_center;
            // let s_x = center - s_center;

            r.extend_left(r_x);
            let line_left = r.bottom_x.min(s_x);
            let line_right = (r.bottom_x + r.bottom_width).max(s_x + s_width);
            let line_width = line_right - line_left;
            let mut line_str: String =
                std::iter::repeat(symbols::Sym::RuleLine.repr(self.conf.unicode))
                    .take(line_width)
                    .collect::<String>();
            line_str += rule.get_symbol().repr(self.conf.unicode);
            r.write_down_at(
                line_str,
                line_width + rule.get_symbol().len(self.conf.unicode),
                line_left,
            );
            r.write_down_at(
                self.proof.sequent.repr_conf(self.conf).to_string(),
                s_width,
                s_x,
            );
            r
        } else {
            let s_width = self.proof.sequent.repr_conf(self.conf).len();
            ProofRender {
                width: s_width,
                height: 1,
                bottom_x: 0,
                bottom_width: s_width,
                buffer: vec![self.proof.sequent.repr_conf(self.conf).to_string()],
            }
        }
    }
}

impl<'a> fmt::Display for ProofRepr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = self.render();
        for y in (0..r.height).rev() {
            r.buffer[y].fmt(f)?;
            "\n".fmt(f)?;
        }
        Ok(())
    }
}

struct ProofRender {
    width: usize,
    height: usize,
    bottom_x: usize,
    bottom_width: usize,
    buffer: Vec<String>,
}
impl ProofRender {
    fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            bottom_x: 0,
            bottom_width: 0,
            buffer: Vec::new(),
        }
    }
    fn extend_right(&mut self, n: usize) {
        self.width += n;
        self.buffer
            .iter_mut()
            .for_each(|l| l.extend(std::iter::repeat(' ').take(n)))
    }
    fn extend_left(&mut self, n: usize) {
        self.width += n;
        self.bottom_x += n;
        let padding: String = std::iter::repeat(' ').take(n).collect();
        self.buffer
            .iter_mut()
            .for_each(|l| l.insert_str(0, &padding))
    }
    fn extend_up(&mut self, n: usize) {
        self.height += n;
        self.buffer
            .extend(std::iter::repeat(std::iter::repeat(' ').take(self.width).collect()).take(n));
    }
    fn push_right(&mut self, rhs: &Self) {
        if self.height < rhs.height {
            self.extend_up(rhs.height - self.height);
        }
        self.bottom_width = self.width - self.bottom_x + rhs.bottom_x + rhs.bottom_width;
        self.width += rhs.width;
        self.buffer
            .iter_mut()
            .zip(rhs.buffer.iter().chain(std::iter::repeat(
                &std::iter::repeat(' ').take(rhs.width).collect(),
            )))
            .for_each(|(l, r)| *l += r);
    }
    fn from_iter(mut iter: impl Iterator<Item = Self>, spacer: usize) -> Self {
        if let Some(mut r) = iter.next() {
            for h in iter {
                if spacer > 0 {
                    r.extend_right(spacer);
                }
                r.push_right(&h);
            }
            r
        } else {
            Self::new()
        }
    }
    fn write_down(&mut self, mut s: String, len: usize) {
        use std::cmp::Ordering;
        match len.cmp(&self.width) {
            Ordering::Greater => self.extend_right(len - self.width),
            Ordering::Less => s.extend(std::iter::repeat(' ').take(self.width - len)),
            _ => {}
        }
        self.height += 1;
        self.buffer.insert(0, s);
    }
    fn write_down_at(&mut self, s: String, len: usize, at: usize) {
        let mut padding: String = std::iter::repeat(' ').take(at).collect();
        padding += &s;
        self.bottom_x = at;
        self.bottom_width = len;
        self.write_down(padding, len + at)
    }
}
