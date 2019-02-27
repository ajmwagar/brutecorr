#![allow(non_snake_case)]

// Setup types and consts
type brute_num = f32;

const RAND_UPPER_LIMIT: usize = 5;
const RAND_LOWER_LIMIT: usize = 0;
const DEBUG: bool = false;

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
                               .help("(Optional) set room for error (eg. 0.1, 0.001). WARNING: making this a very low decimal will cause brutecorr to crash!")
                               .takes_value(true))
                          .arg(Arg::with_name("v")
                               .short("v")
                               .multiple(true)
                               .help("(Optional) Sets the level of verbosity"))
                          .get_matches();

    // Error handling
    let error = matches.value_of("error").unwrap_or("0.1").parse::<brute_num>().unwrap();

    // Warn user of crash
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
    else if error >= 1{
        panic!("Error value is too large. Stopping!")
    }

    // Get target CC
    let target: brute_num = matches.value_of("target").unwrap().parse().unwrap();

    // Get list 1
    let input = matches.value_of("L1").unwrap();

    // Parse in to Vec<brute_num>
    let L1: Vec<brute_num> = input.split_whitespace().map(| word| word.parse::<brute_num>().unwrap_or(0.0)).collect();

    // Check if L1 is longer than 1
    if L1.len() < 1 {
        panic!("Not enough values in L1");
    }

    // Brute force row 2
    let L2 = brute_force_correlation(&L1, target, error, 0 as usize);

    // Print out result
    println!("Row 1: {:?}", L1);
    println!("Row 2: {:?}", L2);
}

/// Brute forcing algorithm for Correralation Coefficents
fn brute_force_correlation(L1: &[brute_num], goal: brute_num, variation: brute_num, counter: usize) -> Vec<brute_num>{
    // Generate random list of numbers
    let L2 = generateL2(L1);

    // println!("L2: {:?}\nlen: {}", L2, L2.len());
    // println!("L1: {:?}\nlen: {}", L1, L1.len());
    if DEBUG{
        println!("Attempt: {}", counter);   
    }

    // get Correralation Coefficents
    let cor = correlation(L1, &L2);

    // check against goal 
    if goal - variation <= cor && cor <= goal + variation {

        // If CC is close to goal then return and tell user
        println!("Result Found!\nTook {} attempts.\nr = {}", counter, cor);
        return L2
    }
    else {
        // Recurse
        return brute_force_correlation(L1, goal, variation, counter + 1)
    }

}

fn generateL2(L1: &[brute_num]) -> Vec<brute_num>{
    // let n = L1.len().to_owned();
    let mut L2: Vec<brute_num> = Vec::new();

    for _i in 0..L1.len() {
        L2.push(thread_rng().gen_range(RAND_LOWER_LIMIT, RAND_UPPER_LIMIT) as brute_num);
    }

    L2
}

fn correlation(listX: &[brute_num], listY: &[brute_num]) -> brute_num {
    let n = listX.len() as brute_num;
    let p = listY.len() as brute_num;

    if n != p {
        panic!("L1 and L2 are not the same length.");
    }

    let Exy: brute_num = maximumSOP(listX, listY);
    let Ex: brute_num = listX.iter().sum();
    let Ey: brute_num = listY.iter().sum();

    let Ex2: brute_num = squareSum(listX);
    let Ey2: brute_num = squareSum(listY);

    return ((n * Exy) - (Ex * Ey)) / ((n * Ex2 - (Ex * Ex)) * ((n * Ey2 - (Ey * Ey)))).sqrt() as brute_num

}

fn squareSum(v: &[brute_num]) -> brute_num {
    v.iter().fold(0., |sum, &num| sum + num*num)
}

fn maximumSOP(listX: &[brute_num], listY: &[brute_num]) -> brute_num {
    let mut sop = 0.0;

    let n = listY.len();

    for i in 0..n {
        sop = sop + listY[i] * listX[i]
    }

    sop

}
