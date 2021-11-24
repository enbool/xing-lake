/*
f(0) = 0
f(1) = 1
f(n) = f(n-1) + f(n-2)


 */

pub(crate) fn fib_loop(n: u8) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    let mut i: u8 = 2;
    loop {
        let c = a + b;
        a = b;
        b = c;
        if i >= n {
            return c;
        }
        i += 1;
    }
}