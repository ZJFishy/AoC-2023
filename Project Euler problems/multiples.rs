use std::io;

fn main(){
    println!("Enter the upper boudn to be considered");
    let mut line_in: String = String::new();

    io::stdin()
        .read_line(&mut line_in)
        .expect("Failed to read line");

    let upper_bound: u32 = line_in.trim().parse().expect("Not a number");
    let mut total = 0;

    for i in 1..upper_bound {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }

    println!("Total is {total}")
}