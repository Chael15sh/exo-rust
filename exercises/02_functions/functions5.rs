// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}
// il attendait soit que l'on précise return soit que l'on enleve le ; pour qu'il comprenne que c'est un return

fn square(num: i32) -> i32 {
    num * num    
}
