use prove::*;

// fn user_input<T>() -> T
// where T: std::str::FromStr
// {
//     let mut buffer = String::new();
//     std::io::stdin().read_line(&mut buffer).unwrap();
//     buffer.trim().parse().map_err(|_| "invalid user input").unwrap()
// }

enum Command {
    Rule(ProveBy),
    Restart,
    Back,
}
impl std::str::FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(':') => {
                match s {
                    ":b" => Ok(Command::Back),
                    ":r" => Ok(Command::Restart),
                    _ => Err("unknown command"),
                }
            }
            Some(_) => Ok(Command::Rule(s.parse()?)),
            None => Err("empty input"),
        }
    }
}

fn user_input_loop<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut buffer = String::new();
    loop {
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        match buffer.trim().parse() {
            Ok(v) => return v,
            Err(e) => eprintln!("invalid user input: {:?}", e),
        }
    }
}

fn press_enter() {
    let _: String = user_input_loop();
}

fn main() {
    let repr_conf = ReprConf{
        negation: true,
        formated: true,
        unicode: false,
        emphazis: false,
    };

    let file = std::io::BufReader::new(std::fs::File::open("sequents.txt").unwrap());
    use std::io::BufRead;
    for line in file.lines() {
        let mut p = Proof::not_proven(line.unwrap().trim().parse().unwrap());
        let mut hist = vec![p.clone()];
        while p.next_not_proven_mut().is_some() {
            println!("\x1b[2J");
            println!("{}", p.repr_conf(repr_conf));
            match user_input_loop::<Command>() {
                Command::Back => {
                    hist.pop();
                    match hist.last() {
                        Some(v) => {
                            p = v.clone();
                        }
                        None => {
                            println!("no more history");
                        }
                    }
                }
                Command::Restart => {
                    hist.drain(1..).count();
                    match hist.last() {
                        Some(v) => {
                            p = v.clone();
                        }
                        None => {
                            println!("no more history");
                        }
                    }
                }
                Command::Rule(rule) => {
                    if !p.prove_next_by(rule) {
                        println!("can't apply rule");
                        press_enter();
                    } else {
                        hist.push(p.clone());
                    }
                }
            }
        }
        println!("\x1b[2J");
        println!("{}", p.repr_conf(repr_conf));
        println!("SOLVED");
        press_enter();
    }
}
