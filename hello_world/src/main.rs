fn pattern_match_simple() {
    let num: i32 = 3;
    let letter: char = match num {
        1 => 'A',
        2 => 'B',
        3 => {'C'},
        _ => '#', // rust will not guess
    };
    println!("{}", letter);
}

fn main() {
    pattern_match_simple();
}
// Try to go through mod 2. 1-3