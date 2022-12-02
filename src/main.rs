mod d2;

fn main() {
    let input = include_str!("../res/d2.txt");
    println!("{}", d2::run(input.to_string()));
}
