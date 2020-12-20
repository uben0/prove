#![recursion_limit="256"]

mod proof;
mod property;
mod sequent;
mod command;

pub use proof::Proof;
pub use property::Prop;
pub use sequent::Sequent;
pub use command::Command;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}