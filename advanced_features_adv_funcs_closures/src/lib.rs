// Contrary to the Fn trait, "fn" is a type denoting function pointers
// You can use them as arguments to functions and closures, so we don't force it to just one type
// Although, one case for this enforcing is when interoperating with a language that doesn't have closures
// For those cases, you just use the Fn traits available in the standard library
pub fn run(f: fn()) {
    f();
}

// The following is not possible, because for function pointers, you must ensure returning a concrete type for the function
// As closures don't have a concrete type, it is illegal to do this
// pub fn run_return_func(f: fn() -> i32) -> fn() {
//     || f()
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn fib(a: i32) -> i32 {
        if a == 0 || a == 1 {
            return a;
        }

        fib(a - 1) + fib(a - 2)
    }

    fn factorial(a: i32) -> i32 {
        if a == 1 {
            return 1;
        }

        factorial(a - 1) * a
    }

    fn factorial_of_five() {
        println!("Factorial of 5 is {}", factorial(5));
    }

    #[test]
    fn it_prints_fib_series() {
        run(|| {
            println!("Fib for {} is {}", 6, fib(6));
        });
    }

    #[test]
    fn it_prints_factorial_series() {
        run(factorial_of_five);
    }
}
