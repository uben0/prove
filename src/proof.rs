use super::property::Prop;
use super::sequent::Sequent;

/// Goes with a sequent as validation
#[derive(Debug, Clone)]
pub enum Proof {
    Hypothesis,
    ImplicationIntro(Sequent),
}
impl Proof {
    pub fn check(&self, sequent: &Sequent, partial: bool) -> bool {
        match self {
            Self::Hypothesis => sequent.hypotheses().contains(sequent.conclusion()),
            Self::ImplicationIntro(o) => match sequent.conclusion() {
                Prop::Implication(lhs, rhs) => {
                    let (lhs, rhs) = (lhs.as_ref(), rhs.as_ref());
                    sequent
                        .hypotheses()
                        .iter()
                        .chain(std::iter::once(lhs))
                        .eq(o.hypotheses())
                        && o.conclusion() == rhs
                        && o.proof().map(|p| p.check(o, partial)).unwrap_or(partial)
                }
                _ => false,
            },
        }
    }
}
