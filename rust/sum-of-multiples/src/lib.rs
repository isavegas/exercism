#![allow(dead_code)]

pub fn sum_of_multiples(limit: i32, multiples: &Vec<i32>) -> i32 {
    let mut vals: Vec<i32> = vec![];

    multiples.iter().for_each(|i| {
        if i < &limit {
            let mut mult: i32 = *i;
            while (mult < limit) {
                if !vals.contains(&mult) {
                    vals.push(mult);
                    println!("{}", mult);
                }
                mult += i;
            }
        }
    });

    return vals.iter().fold(0, |a, b| a + b);
}
