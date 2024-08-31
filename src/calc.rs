pub fn calculate(index: usize) -> u128{
    let mut output: u128 = 0;
    if index == 0{
        return 0
    }

    let mut a: u128 = 1;
    let mut b: u128 = 0;
    let length: usize = index;
    let mut done: usize = 0;
    while done != length{
        done += 1;
        output = a;
        (a, b) = (a + b, a);
    }
    return output;
}