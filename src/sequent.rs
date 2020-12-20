use super::proof::Proof;
use super::property::Prop;

/// Represents any sequent, eg: `A->B, A |- B`
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
    pub fn prove(&mut self, proof: Proof) -> bool {
        if proof.check(self, true) {
            self.proof = Some(Box::new(proof));
            true
        } else {
            false
        }
    }
    pub fn prove_by_hyp(&mut self) -> bool {
        self.prove(Proof::Hypothesis)
    }
    pub fn prove_by_impl_intro(&mut self) -> bool {
        match self.conclusion() {
            Prop::Implication(lhs, rhs) => {
                let o = Sequent::new(
                    self.hypotheses()
                        .iter()
                        .chain(std::iter::once(lhs.as_ref()))
                        .cloned()
                        .collect::<Vec<Prop>>(),
                    rhs.as_ref().clone(),
                );
                self.prove(Proof::ImplicationIntro(o))
            }
            _ => false,
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
    pub fn next_not_proven(&mut self) -> Option<&mut Self> {
        if self.proof.is_some() {
            match self.proof.as_mut().unwrap().as_mut() {
                Proof::Hypothesis => None,
                Proof::ImplicationIntro(o) => o.next_not_proven(),
            }
        } else {
            Some(self)
        }
    }
    pub fn print_proof(&self) {
        let lock = std::io::stdout();
        let mut output = lock.lock();
        render::render_sequent_proof(self, &mut output).unwrap();
    }
}

use std::fmt;
impl fmt::Display for Sequent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for h in &self.hypotheses {
            if !first {
                ", ".fmt(f)?;
            }
            first = false;
            h.fmt(f)?;
        }
        if !first {
            " ".fmt(f)?;
        }
        "|- ".fmt(f)?;
        self.conclusion.fmt(f)
    }
}

use std::str::FromStr;
impl FromStr for Sequent {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split("|-");
        match [iter.next(), iter.next(), iter.next()] {
            [Some(left), Some(right), None] => {
                let hypotheses: Vec<_> = if left.trim().is_empty() {
                    Vec::new()
                }
                else {
                    left
                    .split(',')
                    .map(|h| h.parse())
                    .collect::<Result<_, _>>()?
                };
                Ok(Self::new(hypotheses, right.parse()?))
            }
            [_, None, None] => Err("missing sequent symbol"),
            _ => Err("expecting only one sequent symbol"),
        }
    }
}

mod render {
    use super::{Proof, Sequent};
    use std::{io, io::Write};

    pub fn render_sequent_proof(sequent: &Sequent, output: &mut impl Write) -> io::Result<()> {
        let sg = SequentGeom::from(sequent);
        for y in (0..sg.height).rev() {
            sg.print_line(output, y)?;
            writeln!(output)?;
        }
        Ok(())
    }

    struct SequentGeom {
        bottom: String,
        proof: Option<Box<ProofGeom>>,
        width: usize,
        height: usize,
        bottom_x: usize,
        bottom_width: usize,
        center: usize,
    }
    enum ProofGeom {
        Hyp,
        ImplIntro(SequentGeom),
    }
    impl ProofGeom {
        fn name(&self) -> &'static str {
            match self {
                Self::Hyp => "hyp",
                Self::ImplIntro(_) => "->i",
            }
        }
        fn split_center(&self) -> (usize, usize) {
            match self {
                Self::Hyp => (0, 0),
                Self::ImplIntro(over) => {
                    let center = over.bottom_x + over.bottom_width / 2;
                    (center, over.width - center)
                }
            }
        }
        fn split_bottom(&self) -> (usize, usize) {
            match self {
                Self::Hyp => (0, 0),
                Self::ImplIntro(over) => {
                    let middle = over.bottom_width / 2;
                    (middle, over.bottom_width - middle)
                }
            }
        }
    }
    impl SequentGeom {
        fn from(sequent: &Sequent) -> Self {
            let proof = sequent.proof.as_ref().map(|p| match p.as_ref() {
                Proof::Hypothesis => ProofGeom::Hyp,
                Proof::ImplicationIntro(over) => ProofGeom::ImplIntro(SequentGeom::from(over)),
            });

            let bottom = sequent.to_string();
            let bottom_width = bottom.chars().count();

            if let Some(proof) = proof {
                if let Some((ul_x, ul_width, _u_width, u_height)) = match &proof {
                    ProofGeom::Hyp => None,
                    ProofGeom::ImplIntro(over) => {
                        Some((over.bottom_x, over.bottom_width, over.width, over.height))
                    }
                } {
                    let (u_left, u_right) = proof.split_center();
                    let b_left = bottom_width / 2;
                    let b_right = bottom_width - b_left;
                    let center = u_left.max(b_left);
                    let rule_len = proof.name().chars().count();
                    let rule_end =
                        (center + b_right).max(center - u_left + ul_x + ul_width) + rule_len;
                    let width = (center + u_right.max(b_right)).max(rule_end);
                    let bottom_x = center - b_left;
                    Self {
                        width,
                        height: u_height + 2,
                        bottom_x,
                        bottom_width,
                        center,
                        proof: Some(proof.into()),
                        bottom,
                    }
                } else {
                    Self {
                        width: bottom_width + proof.name().chars().count(),
                        height: 2,
                        bottom_x: 0,
                        center: bottom_width / 2,
                        bottom_width,
                        proof: Some(proof.into()),
                        bottom,
                    }
                }
            } else {
                Self {
                    width: bottom_width,
                    center: bottom_width / 2,
                    height: 1,
                    bottom_x: 0,
                    bottom_width,
                    bottom,
                    proof: None,
                }
            }
        }
        fn print_line(&self, out: &mut impl Write, line: usize) -> io::Result<()> {
            match line {
                0 => {
                    for _ in 0..self.bottom_x {
                        write!(out, " ")?;
                    }
                    write!(out, "{}", self.bottom)?;
                    for _ in (self.bottom_x + self.bottom_width)..self.width {
                        write!(out, " ")?;
                    }
                    Ok(())
                }
                1 => {
                    let proof = self.proof.as_ref().unwrap().as_ref();
                    let (ub_left, ub_right) = proof.split_bottom();
                    let b_left = self.bottom_width / 2;
                    let b_right = self.bottom_width - b_left;
                    let l_begin = self.center - b_left.max(ub_left);
                    let l_end = self.center + b_right.max(ub_right);
                    for _ in 0..l_begin {
                        write!(out, " ")?;
                    }
                    for _ in l_begin..l_end {
                        write!(out, "─")?;
                    }
                    write!(out, "{}", proof.name());
                    for _ in (l_end + proof.name().chars().count())..self.width {
                        write!(out, " ")?;
                    }
                    Ok(())
                }
                _ => {
                    let proof = self.proof.as_ref().unwrap().as_ref();
                    match proof {
                        ProofGeom::ImplIntro(over) => {
                            let line = line - 2;
                            let (u_left, u_right) = proof.split_center();
                            for _ in 0..(self.center - u_left) {
                                write!(out, " ")?;
                            }
                            over.print_line(out, line)?;
                            for _ in (self.center + u_right)..self.width {
                                write!(out, " ")?;
                            }
                            Ok(())
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
}
