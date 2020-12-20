use super::property::Prop;
use super::sequent::Sequent;
use super::command::Command;
use super::label;

/// Goes with a sequent as validation
#[derive(Debug, Clone)]
pub enum Proof {
    Hypothesis([Sequent; 0]),
    ImplicationIntro([Sequent; 1]),
    ImplicationIntros([Sequent; 1]),
    DisjonctionElim([Sequent; 3]),
    DisjonctionIntroL([Sequent; 1]),
    DisjonctionIntroR([Sequent; 1]),
    ConjonctionElim([Sequent; 2]),
    ConjonctionIntro([Sequent; 2]),
    Exfalso([Sequent; 1]),
    ModusPonens([Sequent; 2]),
    Weakened([Sequent; 1]),
}
impl Proof {
    pub fn hypothesis(sequent: &Sequent) -> Option<Self> {
        if sequent.hypotheses().contains(sequent.conclusion()) {
            Some(Self::Hypothesis([]))
        } else {
            None
        }
    }
    pub fn impl_intro(sequent: &Sequent) -> Option<Self> {
        match sequent.conclusion() {
            Prop::Implication(lhs, rhs) => Some(Self::ImplicationIntro([Sequent::new(
                sequent
                    .hypotheses()
                    .iter()
                    .chain(std::iter::once(lhs.as_ref()))
                    .cloned()
                    .collect::<Vec<Prop>>(),
                rhs.as_ref().clone(),
            )])),
            _ => None,
        }
    }
    pub fn impl_intros(sequent: &Sequent) -> Self {
        let mut s = sequent.clone();
        while let Prop::Implication(lhs, rhs) = s.conclusion() {
            s = Sequent::new(s.hypotheses().iter().chain(std::iter::once(lhs.as_ref())).cloned().collect(), rhs.as_ref().clone());
        }
        Self::ImplicationIntros([s])
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
        Self::DisjonctionElim([a_or_b, with_a, with_b])
    }
    pub fn disj_i_l(sequent: &Sequent) -> Option<Self> {
        match sequent.conclusion() {
            Prop::Disjonction(lhs, _) => Some(Proof::DisjonctionIntroL([Sequent::new(
                sequent.hypotheses().to_owned(),
                lhs.as_ref().clone(),
            )])),
            _ => None,
        }
    }
    pub fn disj_i_r(sequent: &Sequent) -> Option<Self> {
        match sequent.conclusion() {
            Prop::Disjonction(_, rhs) => Some(Proof::DisjonctionIntroR([Sequent::new(
                sequent.hypotheses().to_owned(),
                rhs.as_ref().clone(),
            )])),
            _ => None,
        }
    }
    pub fn conj_i(sequent: &Sequent) -> Option<Self> {
        match sequent.conclusion() {
            Prop::Conjonction(lhs, rhs) => Some(Proof::ConjonctionIntro([
                Sequent::new(sequent.hypotheses().to_owned(), lhs.as_ref().clone()),
                Sequent::new(sequent.hypotheses().to_owned(), rhs.as_ref().clone()),
            ])),
            _ => None,
        }
    }
    pub fn conj_e(sequent: &Sequent, a: &Prop, b: &Prop) -> Self {
        Self::ConjonctionElim([
            Sequent::new(sequent.hypotheses().to_owned(), a.and(b)),
            Sequent::new(
                sequent
                    .hypotheses()
                    .iter()
                    .chain([a, b].iter().copied())
                    .cloned()
                    .collect(),
                sequent.conclusion().clone(),
            ),
        ])
    }
    pub fn exfalso(sequent: &Sequent) -> Self {
        Self::Exfalso([Sequent::new(sequent.hypotheses().to_owned(), Prop::False)])
    }
    pub fn modus_ponens(sequent: &Sequent, b: &Prop) -> Self {
        Self::ModusPonens([
            Sequent::new(
                sequent.hypotheses().to_owned(),
                b.implies(sequent.conclusion()),
            ),
            Sequent::new(sequent.hypotheses().to_owned(), b.clone()),
        ])
    }
    pub fn weakened(sequent: &Sequent, e: &[usize]) -> Option<Self> {
        let mut filtered = false;
        let r = Self::Weakened([Sequent::new(
            sequent
                .hypotheses()
                .iter()
                .enumerate()
                .filter(|(i, _)| if e.contains(i) {filtered = true; false} else {true})
                .map(|(_, p)| p)
                .cloned()
                .collect(),
            sequent.conclusion().clone(),
        )]);
        if filtered {
            Some(r)
        } else {
            None
        }
    }
    pub fn apply(sequent: &Sequent, e: usize) -> Option<Self> {
        sequent.hypotheses().get(e).map(|e| match e {
            Prop::Implication(lhs, rhs) => if rhs.as_ref() == sequent.conclusion() {
                let mut mp = Self::modus_ponens(sequent, lhs.as_ref());
                mp.array_mut()[0].prove_by(Command::Hypothesis);
                Some(mp)
            } else {
                None
            }
            _ => None,
        }).flatten()
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Hypothesis(_) => label::HYPOTHESIS,
            Self::ImplicationIntro(_) => label::IMPLICATION_I,
            Self::ImplicationIntros(_) => label::IMPLICATION_IS,
            Self::DisjonctionElim(_) => label::DISJONCTION_E,
            Self::DisjonctionIntroL(_) => label::DISJONCTION_I_L,
            Self::DisjonctionIntroR(_) => label::DISJONCTION_I_R,
            Self::ConjonctionElim(_) => label::CONJONCTION_E,
            Self::ConjonctionIntro(_) => label::CONJONCTION_I,
            Self::Exfalso(_) => label::FALSE_E,
            Self::ModusPonens(_) => "mp",
            Self::Weakened(_) => "aff",
        }
    }
    pub fn array(&self) -> &[Sequent] {
        match self {
            Self::Hypothesis(a) => a,
            Self::ImplicationIntro(a) => a,
            Self::ImplicationIntros(a) => a,
            Self::DisjonctionElim(a) => a,
            Self::DisjonctionIntroL(a) => a,
            Self::DisjonctionIntroR(a) => a,
            Self::ConjonctionElim(a) => a,
            Self::ConjonctionIntro(a) => a,
            Self::Exfalso(a) => a,
            Self::ModusPonens(a) => a,
            Self::Weakened(a) => a,
        }
    }
    pub fn array_mut(&mut self) -> &mut [Sequent] {
        match self {
            Self::Hypothesis(a) => a,
            Self::ImplicationIntro(a) => a,
            Self::ImplicationIntros(a) => a,
            Self::DisjonctionElim(a) => a,
            Self::DisjonctionIntroL(a) => a,
            Self::DisjonctionIntroR(a) => a,
            Self::ConjonctionElim(a) => a,
            Self::ConjonctionIntro(a) => a,
            Self::Exfalso(a) => a,
            Self::ModusPonens(a) => a,
            Self::Weakened(a) => a,
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
