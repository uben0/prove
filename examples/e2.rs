use prove::*;

fn get_cmd(buffer: &mut String) -> Command {
    loop {
        buffer.clear();
        std::io::stdin().read_line(buffer).unwrap();
        match buffer.trim().parse() {
            Ok(cmd) => return cmd,
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}

fn main() {
    // |- P \/ Q -> (P->!) -> Q
    // |- (A -> B) /\ (B -> A) -> (B -> A) /\ (A -> B)
    // Q\/(Q->!), P->Q->R, P->(Q->!)->S |- P->(R\/S)


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
            let c = get_cmd(&mut buffer);
            if !not_proven.prove_by(c) {
                println!("invalid application");
                continue
            }
        }
        else {
            return
        }
    }
}