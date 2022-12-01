mod d1;

fn main() {
    let input = include_str!("../res/d1.txt");
    println!("{}", d1::run(input.to_string()));
}
