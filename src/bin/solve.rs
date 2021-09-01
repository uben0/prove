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
    Skip,
    Help,
    Negation,
    Quit,
}
impl std::str::FromStr for Command {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some(':') => {
                match s {
                    ":b" => Ok(Command::Back),
                    ":r" => Ok(Command::Restart),
                    ":s" => Ok(Command::Skip),
                    ":h" => Ok(Command::Help),
                    ":n" => Ok(Command::Negation),
                    ":q" => Ok(Command::Quit),
                    _ => Err("     unknown command     "),
                }
            }
            Some(_) => Ok(Command::Rule(s.parse()?)),
            None => Err("       empty input       "),
        }
    }
}

fn print_usage() {
    clear_screen();
    println!("\x1b[7mCOMMANDS                                                                        \x1b[0m");
    println!("  :b            back one step, undo the last action");
    println!("  :r            reset all steps, undo all actions");
    println!("  :s            skip to the next sequent");
    println!("  :h            print this help message");
    println!("  :n            toggle on/off the negation representaion");
    println!("  :q            quit the program");
    println!();
    println!("\x1b[7mAPPLICABLE RULES                                                                \x1b[0m");
    println!("  h             hypothesis");
    println!("  i             introduction of the conclusion (automatic: it choses");
    println!("                introduction rule base on conclusion type)");
    println!("  xf            exflaso");
    println!("  e <N>         elimination of the Nth hypothesis (automatic: it choses");
    println!("                elimination rule base on hypothesis type)");
    println!("  ii            implication introduction");
    println!("  iis           implications introduction (for chaining implications)");
    println!("  dil           disjonction introduction left");
    println!("  dir           disjonction introduction right");
    println!("  mp <F>        modus ponens on F (a logical property formula like: ~P/\\Q)");
    println!("  de <F>, <F>   disjonction elimination of left formula and right formula");
    println!("  ce <F>, <F>   conjonction elimination of left formula and right formula");
    println!();
    press_enter("           ok            ", "\x1b[94m");
}

fn clear_screen() {
    println!("\x1b[H\x1b[2J\x1b[3J");
}

fn try_user_input<T: std::str::FromStr>() -> Result<T, <T as std::str::FromStr>::Err> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse()
}

fn press_enter(message: &str, color: &str) {
    println!("\x1b[7m{}{}\x1b[0m", color, message);
    println!("\x1b[7;1m[      press enter      ]\x1b[0m");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}

fn main() {
    print_usage();

    let mut repr_conf = ReprConf{
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
            clear_screen();
            println!("{}", p.repr_conf(repr_conf));
            match try_user_input::<Command>() {
                Ok(Command::Skip) => {
                    break
                }
                Ok(Command::Back) => {
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
                Ok(Command::Restart) => {
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
                Ok(Command::Help) => {
                    print_usage();
                }
                Ok(Command::Negation) => {
                    repr_conf.negation = !repr_conf.negation;
                }
                Ok(Command::Quit) => {
                    return
                }
                Ok(Command::Rule(rule)) => {
                    if !p.prove_next_by(rule) {
                        press_enter("     can't apply rule    ", "\x1b[91m");
                    } else {
                        hist.push(p.clone());
                    }
                }
                Err(e) => {
                    press_enter(&e, "\x1b[91m");
                }
            }
        }
        clear_screen();
        println!("{}", p.repr_conf(repr_conf));
        println!();
        press_enter("         SOLVED          ", "\x1b[94m");
    }
}
