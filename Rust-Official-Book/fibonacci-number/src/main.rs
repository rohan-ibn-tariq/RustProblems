fn main() {

    let mut n = 10 ;
    let mut fib_n = nth_fibonacci(n);
    println!("The {n}th Fibonacci number is: {fib_n}");

    n = 35 ;
    fib_n = nth_fibonacci(n);
    println!("The {n}th Fibonacci number is: {fib_n}");
}

fn nth_fibonacci( n: u64 ) -> u64 {
    if n == 0 {
        0
    } else if n == 1 { 
        1   
    } else {
        nth_fibonacci(n-1) + nth_fibonacci(n-2)
    }
}