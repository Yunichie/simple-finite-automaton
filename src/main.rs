use finite_automaton::fa::dfa::DFA;
use std::io;

fn main() {
    /*
     * Immediately initialize sigma
     * let mut sigma = String::from("aaabbb");
     *
     * or read from user input by first initializing an empty String
     */
    let mut sigma = String::new();
    // read user input for sigma
    io::stdin().read_line(&mut sigma);

    let dfa = DFA::new(sigma);
    println!("{}", dfa.run());
}
