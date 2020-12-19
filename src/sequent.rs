use super::proof::Proof;
use super::property::Prop;

/// Represents any sequent, eg: `A/\B |- B`
#[derive(Debug, Clone)]
pub struct Sequent {
    hypotheses: Vec<Prop>,
    conclusion: Prop,
    proof: Option<Box<Proof>>,
}

impl Sequent {
    pub fn new(hypotheses: Vec<Prop>, conclusion: Prop) -> Self {
        Self {
            hypotheses,
            conclusion,
            proof: None,
        }
    }
    pub fn prove(self, proof: Proof) -> Option<Self> {
        if proof.check(&self, true) {
            Some(Self{
                proof: Some(Box::new(proof)),
                .. self
            })
        }
        else {
            None
        }
    }
    pub fn hypotheses(&self) -> &[Prop] {
        &self.hypotheses
    }
    pub fn conclusion(&self) -> &Prop {
        &self.conclusion
    }
    pub fn proof(&self) -> Option<&Proof> {
        self.proof.as_ref().map(|p| p.as_ref())
    }
}
