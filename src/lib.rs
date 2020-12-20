mod sym {
    // pub const DISJONCTION: &str = "\\/";
    // pub const CONJONCTION: &str = "/\\";
    // pub const SEQUENT: &str = "|-";
    // pub const IMPLICATION: &str = "->";
    // pub const FALSE: &str = "!";

    pub const DISJONCTION: &str = "∨";
    pub const CONJONCTION: &str = "∨";
    pub const SEQUENT: &str = "⊢";
    pub const IMPLICATION: &str = "➔";
    pub const FALSE: &str = "⊥";
}
mod label {
    // pub const DISJONCTION_E: &str = "\\/e";
    // pub const DISJONCTION_I_L: &str = "\\/i,l";
    // pub const DISJONCTION_I_R: &str = "\\/i,r";
    // pub const CONJONCTION_E: &str = "/\\e";
    // pub const CONJONCTION_I: &str = "/\\i";
    // pub const IMPLICATION_I: &str = "->i";
    // pub const FALSE_E: &str = "!e";

    pub const HYPOTHESIS: &str = "h";
    pub const DISJONCTION_E: &str = "∨e";
    pub const DISJONCTION_I_L: &str = "∨i,l";
    pub const DISJONCTION_I_R: &str = "∨i,r";
    pub const CONJONCTION_E: &str = "∧e";
    pub const CONJONCTION_I: &str = "∧i";
    pub const IMPLICATION_I: &str = "➔i";
    pub const FALSE_E: &str = "⊥e";
}
// ("∧", "∨", "➔")

mod command;
mod proof;
mod property;
mod sequent;

pub use command::Command;
pub use proof::Proof;
pub use property::Prop;
pub use sequent::Sequent;
