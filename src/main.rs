use rand::{thread_rng, Rng};

fn main() {
    println!("{:?}", brute_force_correlation(&[2.0, 2.3, 3.3, 3.7, 4.2, 4.6, 4.5, 5., 5.5, 5.7, 6.1, 6.4], 0.75, 0.001, 0 as usize))
}

fn brute_force_correlation(L1: &[f64], goal: f64, variation: f64, counter: usize) -> Vec<f64>{
    let L2 = generateL2(L1);

    // println!("L2: {:?}\nlen: {}", L2, L2.len());
    // println!("L1: {:?}\nlen: {}", L1, L1.len());

    let cor = correlation(L1, &L2);


    if goal - variation <= cor && cor <= goal + variation {
        println!("Attempt {}: {}", counter, cor);
        return L2
    }
    else {
        return brute_force_correlation(L1, goal, variation, counter + 1)
    }

}

fn generateL2(L1: &[f64]) -> Vec<f64>{
    // let n = L1.len().to_owned();
    let mut L2: Vec<f64> = Vec::new();

    for _i in 0..L1.len() {
        L2.push(thread_rng().gen_range(0, 100) as f64);
    }

    L2
}

fn correlation(listX: &[f64], listY: &[f64]) -> f64 {
    let n = listX.len() as f64;
    let p = listY.len() as f64;

    if n != p {
        panic!("L1 and L2 are not the same length.");
    }

    let Exy: f64 = maximumSOP(listX, listY);
    let Ex: f64 = listX.iter().sum();
    let Ey: f64 = listY.iter().sum();

    let Ex2: f64 = squareSum(listX);
    let Ey2: f64 = squareSum(listY);

    return ((n * Exy) - (Ex * Ey)) / ((n * Ex2 - (Ex * Ex)) * ((n * Ey2 - (Ey * Ey)))).sqrt() as f64

}

fn squareSum(v: &[f64]) -> f64 {
    v.iter().fold(0., |sum, &num| sum + num*num)
}

fn maximumSOP(listX: &[f64], listY: &[f64]) -> f64 {
    let mut sop = 0.0;

    let n = listY.len();

    for i in 0..n {
        sop = sop + listY[i] * listX[i]
    }

    sop

}
