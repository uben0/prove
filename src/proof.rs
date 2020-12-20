use super::property::Prop;
use super::sequent::Sequent;

/// Goes with a sequent as validation
#[derive(Debug, Clone)]
pub enum Proof {
    Hypothesis,
    ImplicationIntro(Sequent),
    DisjonctionElim(Sequent, Sequent, Sequent),
    Exfalso(Sequent),
}
impl Proof {
    pub fn hypothesis(sequent: &Sequent) -> Option<Self> {
        if sequent.hypotheses().contains(sequent.conclusion()) {
            Some(Self::Hypothesis)
        } else {
            None
        }
    }
    pub fn impl_intro(sequent: &Sequent) -> Option<Self> {
        match sequent.conclusion() {
            Prop::Implication(lhs, rhs) => Some(Self::ImplicationIntro(Sequent::new(
                sequent
                    .hypotheses()
                    .iter()
                    .chain(std::iter::once(lhs.as_ref()))
                    .cloned()
                    .collect::<Vec<Prop>>(),
                rhs.as_ref().clone(),
            ))),
            _ => None,
        }
    }
    pub fn disj_elim(sequent: &Sequent, a: &Prop, b: &Prop) -> Self {
        let a_or_b = Sequent::new(sequent.hypotheses().to_owned(), a.or(b));
        let with_a = Sequent::new(
            sequent
                .hypotheses()
                .iter()
                .chain(std::iter::once(a))
                .cloned()
                .collect(),
            sequent.conclusion().clone(),
        );
        let with_b = Sequent::new(
            sequent
                .hypotheses()
                .iter()
                .chain(std::iter::once(b))
                .cloned()
                .collect(),
            sequent.conclusion().clone(),
        );
        Self::DisjonctionElim(a_or_b, with_a, with_b)
    }
    pub fn exfalso(sequent: &Sequent) -> Self {
        Self::Exfalso(Sequent::new(sequent.hypotheses().to_owned(), Prop::False))
    }
    pub fn iter(&self) -> ProofIter {
        ProofIter { proof: self, n: 0 }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Hypothesis => "hyp",
            Self::ImplicationIntro(_) => "->i",
            Self::DisjonctionElim(_, _, _) => "\\/e",
            Self::Exfalso(_) => "!e",
        }
    }
}

pub struct ProofIter<'a> {
    proof: &'a Proof,
    n: usize,
}
impl<'a> Iterator for ProofIter<'a> {
    type Item = &'a Sequent;
    fn next(&mut self) -> Option<Self::Item> {
        match self.proof {
            Proof::Hypothesis => None,
            Proof::ImplicationIntro(o0) => match self.n {
                0 => {
                    self.n += 1;
                    Some(o0)
                }
                _ => None,
            },
            Proof::DisjonctionElim(o0, o1, o2) => match self.n {
                0 => {
                    self.n += 1;
                    Some(o0)
                }
                1 => {
                    self.n += 1;
                    Some(o1)
                }
                2 => {
                    self.n += 1;
                    Some(o2)
                }
                _ => None,
            },
            Proof::Exfalso(o0) => match self.n {
                0 => {
                    self.n += 1;
                    Some(o0)
                }
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn iter_eq() {
        assert!(![0, 1, 2].iter().eq([0, 1].iter()));
        assert!(![0, 1].iter().eq([0, 1, 2].iter()));
    }
    #[test]
    fn slice_eq() {
        assert!(!([0, 1, 2][..] == [0, 1][..]));
        assert!(!([0, 1][..] == [0, 1, 2][..]));
    }
}
