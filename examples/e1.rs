// use proof::*;

fn main() {
    // let a = Prop::var("A");
    // let a_a = a.implies(&a);

    // // A |- A
    // let a_s_a = Sequent::new(vec![a.clone()], a)
    //     .prove(Proof::Hypothesis)
    //     .unwrap();

    // // |- A->A
    // let s_a_a = Sequent::new(vec![], a_a)
    //     .prove(Proof::ImplicationIntro(a_s_a))
    //     .unwrap();

    // s_a_a.print_proof();
    // println!("{:#?}", s_a_a);

    // let mut buffer = String::new();
    // loop {
    //     buffer.clear();
    //     std::io::stdin().read_line(&mut buffer).unwrap();
    //     let p: Sequent = buffer.trim().parse().unwrap();
    //     println!("{:#?}", p);
    //     println!("-------------------------");
    //     println!("{:}", p);
    //     println!("-------------------------");
    //     p.print_proof();
    // }
}
