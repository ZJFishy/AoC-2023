fn main() {
    println!("{}", largest_factor(600851475143));
}

fn check_prime(num_in: u64) -> bool {
    for i in 2..num_in {
        if num_in % i == 0 {
            return false;
        }
    }
    return true;
}

fn largest_factor(num_in: u64) -> u64 {
    let mut temp = 0;

    for i in 1..((num_in as f64).sqrt() as u64) {
        if i < temp {
            return temp;
        }

        if num_in % i == 0 {
            let mut largest = 0;
            if check_prime(i) == true {largest = i;}
            else {largest = largest_factor(i);}
            if temp < largest {
                temp = largest;
            }
        }
    }

    return temp;
}