use super::command::Command;
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
    pub fn prove_by(&mut self, cmd: Command) -> bool {
        if let Some(proof) = match cmd {
            Command::Hypothesis => Proof::hypothesis(self),
            Command::IntroImplication => Proof::impl_intro(self),
            Command::ElimDisjonction(a, b) => Some(Proof::disj_elim(self, &a, &b)),
            Command::Exfalso => Some(Proof::exfalso(self)),
            Command::ModusPonens(b) => Some(Proof::modus_ponens(self, &b)),
            Command::ElimConjonction(a, b) => Some(Proof::conj_e(self, &a, &b)),
            Command::IntroConjonction => Proof::conj_i(self),
            Command::IntroDisjonctionL => Proof::disj_i_l(self),
            Command::IntroDisjonctionR => Proof::disj_i_r(self),
            Command::Weakened(e) => Proof::weakened(self, &e),
        } {
            self.proof = Some(proof.into());
            true
        } else {
            false
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
            let array = self.proof.as_mut().unwrap().array_mut();
            for o in array {
                if let Some(next_not_proven) = o.next_not_proven() {
                    return Some(next_not_proven);
                }
            }
            None
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
                } else {
                    left.split(',')
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
    use super::Sequent;
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
        proof: Option<Box<ProofRepr>>,
        width: usize,
        height: usize,
        bottom_x: usize,
        bottom_width: usize,
        center: usize,
    }
    struct ProofRepr {
        over: Vec<SequentGeom>,
        name: String,
    }
    impl ProofRepr {
        fn line_x(&self) -> Option<usize> {
            self.over.split_first().map(|(f, _)| f.bottom_x)
        }
        fn line_width(&self) -> Option<usize> {
            self.over.split_first().map(|(f, r)| {
                r.split_last()
                    .map(|(l, r)| {
                        r.iter().map(|o| o.width + 4).sum::<usize>()
                            + (f.width - f.bottom_x)
                            + 4
                            + (l.bottom_x + l.bottom_width)
                    })
                    .unwrap_or(f.bottom_width)
            })
        }
        fn width(&self) -> Option<usize> {
            self.over
                .split_first()
                .map(|(f, r)| f.width + r.iter().map(|o| o.width + 4).sum::<usize>())
        }
        fn height(&self) -> Option<usize> {
            self.over.iter().map(|o| o.height).max()
        }
        fn split_center(&self) -> (usize, usize) {
            let center = self.line_x().unwrap_or(0) + self.line_width().unwrap_or(0) / 2;
            (center, self.width().unwrap_or(0) - center)
        }
        fn split_bottom(&self) -> (usize, usize) {
            let line_width = self.line_width().unwrap_or(0);
            let middle = line_width / 2;
            (middle, line_width - middle)
        }
    }
    impl SequentGeom {
        fn from(sequent: &Sequent) -> Self {
            let proof_repr = sequent.proof.as_ref().map(|p| ProofRepr {
                over: p.array().iter().map(|s| SequentGeom::from(s)).collect(),
                name: p.label().to_owned(),
            });

            let bottom = sequent.to_string();
            let bottom_width = bottom.chars().count();

            if let Some(proof) = proof_repr {
                if let (Some(ul_x), Some(ul_width), Some(_u_width), Some(u_height)) = (
                    proof.line_x(),
                    proof.line_width(),
                    proof.width(),
                    proof.height(),
                ) {
                    let (u_left, u_right) = proof.split_center();
                    let b_left = bottom_width / 2;
                    let b_right = bottom_width - b_left;
                    let center = u_left.max(b_left);
                    let rule_len = proof.name.chars().count();
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
                        width: bottom_width + proof.name.chars().count(),
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
                    if let Some(proof) = self.proof.as_ref() {
                        let proof = proof.as_ref();
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
                        write!(out, "{}", proof.name)?;
                        for _ in (l_end + proof.name.chars().count())..self.width {
                            write!(out, " ")?;
                        }
                        Ok(())
                    } else {
                        for _ in 0..self.width {
                            write!(out, " ")?;
                        }
                        Ok(())
                    }
                }
                _ => {
                    if let Some(proof) = self.proof.as_ref() {
                        let proof = proof.as_ref();
                        let line = line - 2;
                        let (u_left, u_right) = proof.split_center();
                        for _ in 0..(self.center - u_left) {
                            write!(out, " ")?;
                        }
                        let mut first = true;
                        for o in &proof.over {
                            if !first {
                                write!(out, "    ")?;
                            }
                            first = false;
                            o.print_line(out, line)?;
                        }
                        for _ in (self.center + u_right)..self.width {
                            write!(out, " ")?;
                        }
                        Ok(())
                    } else {
                        for _ in 0..self.width {
                            write!(out, " ")?;
                        }
                        Ok(())
                    }
                }
            }
        }
    }
}
