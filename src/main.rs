#![allow(non_snake_case)]

extern crate rand;
use rand::{thread_rng, Rng};
extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("brutecorr")
                          .version("1.0")
                          .author("Avery Wagar <ajmw.subs@gmail.com>")
                          .about("Brute force Pearson Correlational Coeffiecents")
                          .arg(Arg::with_name("target")
                               .short("t")
                               .long("target")
                               .value_name("TARGET")
                               .help("set the Pearson Correlational Coeffiecent")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("L1")
                               // .short("l")
                               // .long("list")
                               .value_name("L1")
                               // .help("set L1")
                               .takes_value(true)
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("error")
                               .short("e")
                               .long("error")
                               .value_name("ERROR")
                               .help("(Optional) set room for error (eg. 0.1, 0.001). printlnING: making this a very low decimal will cause brutecorr to crash!")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("(Optional) Sets the level of verbosity"))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let error = matches.value_of("error").unwrap_or("0.1").parse::<f64>().unwrap();

    if error <= 0.0 {
        panic!("Error value is too small. Stoppping!");
    }
    else if error <= 0.00001 {
        println!("Low error value, crash is imminent!")
    }
    else if error <=  0.0001 {
        println!("Low error value, crash is very likely!")
    }
    else if error <= 0.001 {
        println!("Low error value, crash is likely!")
    }
    else if error < 0.01 {
        println!("Low error value, crash is possible.");
    }

    let target: f64 = matches.value_of("target").unwrap().parse().unwrap();

    let input = matches.value_of("L1").unwrap();

    let L1: Vec<f64> = input.split_whitespace().map(| word| word.parse::<f64>().unwrap_or(0.0)).collect();

    if L1.len() < 1 {
        panic!("Not enough values in L1");
    }


    println!("{:?}", brute_force_correlation(&L1, target, error, 0 as usize))
}

fn brute_force_correlation(L1: &[f64], goal: f64, variation: f64, counter: usize) -> Vec<f64>{
    let L2 = generateL2(L1);

    // println!("L2: {:?}\nlen: {}", L2, L2.len());
    // println!("L1: {:?}\nlen: {}", L1, L1.len());

    let cor = correlation(L1, &L2);

    println!("Attempt {}: {}", counter, cor);

    if goal - variation <= cor && cor <= goal + variation {
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
