
fn main() {
    println!("{}", recursive_power(2, 127));
}

/*
    x^n kan defineres ved
    x^n = {
        1           når n == 0
        x*x^(n-1)   når n > 0
    }
    Programmer dette ved hjelp av rekursjon
*/
fn recursive_power(x: u128, n: u128) -> u128 {
    fn tail_recursive_power(x: u128, n: u128, acc: u128) -> u128 {
        if n == 1 {
            return acc;
        }
        tail_recursive_power(x, n - 1, acc * x)
    }

    if n == 0 {
        return 0;
    } else if n == 1 {
        return x;
    }
    tail_recursive_power(x, n, x)
}
