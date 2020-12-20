use proof::*;

fn main() {
    let mut buffer = String::new();
    println!("enter a sequent to prove:");
    buffer.clear();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut s: Sequent = buffer.trim().parse().unwrap();

    loop {
        println!("\x1b[2J");
        s.print_proof();
        if let Some(not_proven) = s.next_not_proven() {
            println!();
            println!();
            println!("prove: {}", not_proven);
            buffer.clear();
            std::io::stdin().read_line(&mut buffer).unwrap();
            let c: Command = buffer.trim().parse().unwrap();
            if !c.apply_on(not_proven) {
                println!("invalid application");
                continue;
            }
        }
        else {
            return ();
        }
    }
}