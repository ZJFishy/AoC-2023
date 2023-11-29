fn main(){
    let mut fib_1 = 0;
    let mut fib_2 = 1;
    let mut total = 0;

    while fib_1 + fib_2 < 4_000_000 {
        let new_fib = fib_1 + fib_2;

        if new_fib % 2 == 0{
            total += new_fib
        }
        
        fib_1 = fib_2;
        fib_2 = new_fib;
    }

    println!("Total is {total}")
}