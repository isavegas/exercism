pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0;
    let mut len = 0;

    let mut n = num;
    while n > 0 {
        len += 1;
        n = n / 10;
    }

    let mut n = num;
    while n > 0 {
        sum += (n % 10).pow(len);
        n = n / 10;
    }

    sum == num
}
