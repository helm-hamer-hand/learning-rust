
pub fn fibonacci(n: i128) -> i128 {
    let mut f_nmin2: i128 = 0;
    let mut f_nmin1: i128 = 1;
    let mut f_n: i128 = 0;

    for i in 0..n {
        f_n = f_nmin1 + f_nmin2;

        f_nmin2 = f_nmin1;
        f_nmin1 = f_n;
        println!("The {}-th fibonacci number = {f_n}", i+2);
    }
    f_n
}