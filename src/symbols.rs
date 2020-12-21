pub mod lex {
    pub const DISJONCTION: &str = "\\/";
    pub const CONJONCTION: &str = "/\\";
    pub const SEQUENT: &str = "|-";
    pub const COMMA: &str = ",";
    pub const IMPLICATION: &str = "->";
    pub const FALSE: &str = "!";
    pub const PARENTHESIS_OPEN: &str = "(";
    pub const PARENTHESIS_CLOSE: &str = ")";
}

#[cfg(not(feature = "unicode"))]
pub mod repr {
    pub const DISJONCTION: &str = "\\/";
    pub const CONJONCTION: &str = "/\\";
    pub const SEQUENT: &str = "|-";
    pub const IMPLICATION: &str = "->";
    pub const FALSE: &str = "!";
    pub const PARENTHESIS_OPEN: &str = "(";
    pub const PARENTHESIS_CLOSE: &str = ")";

    pub const HYPOTHESIS: &str = "hyp";
    pub const DISJONCTION_E: &str = "\\/e";
    pub const DISJONCTION_I_L: &str = "\\/i,l";
    pub const DISJONCTION_I_R: &str = "\\/i,r";
    pub const CONJONCTION_E: &str = "/\\e";
    pub const CONJONCTION_I: &str = "/\\i";
    pub const IMPLICATION_I: &str = "->i";
    pub const IMPLICATION_IS: &str = "->i'";
    pub const FALSE_E: &str = "!e";
}
#[cfg(feature = "unicode")]
pub mod repr {
    pub const DISJONCTION: &str = "∨";
    pub const CONJONCTION: &str = "∨";
    pub const SEQUENT: &str = "⊢";
    pub const IMPLICATION: &str = "➔";
    pub const FALSE: &str = "⊥";
    pub const PARENTHESIS_OPEN: &str = "(";
    pub const PARENTHESIS_CLOSE: &str = ")";

    pub const HYPOTHESIS: &str = "h";
    pub const DISJONCTION_E: &str = "∨e";
    pub const DISJONCTION_I_L: &str = "∨i,l";
    pub const DISJONCTION_I_R: &str = "∨i,r";
    pub const CONJONCTION_E: &str = "∧e";
    pub const CONJONCTION_I: &str = "∧i";
    pub const IMPLICATION_I: &str = "➔i";
    pub const IMPLICATION_IS: &str = "➔i'";
    pub const FALSE_E: &str = "⊥e";
}
