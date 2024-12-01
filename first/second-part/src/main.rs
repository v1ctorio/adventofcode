use std::fs;

fn main() {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<&str> = input.split("\n").collect();
    for line in input {
        if line == "" {
            continue;
        }
        let line: Vec<&str> = line.split("   ").collect();
        let left = line[0];
        let right = line[1];

        first.push(left.parse().unwrap());
        second.push(right.parse().unwrap());
    }

    let mut sum = 0;
    println!("{:?}", first.len());

    for i in 0..first.len() {
        let left = first[i];

        let times = how_many_time_is_in(&left, &second);

        let mult = left * times;
        sum = sum + mult;
        println!("The times {} is on second is {}", left, times);
    }

    println!("The sum of all differences is {}", sum);
}

fn how_many_time_is_in(n: &u32, vec: &Vec<u32>) -> u32 {
    let mut times = 0;

    for v in vec {
        if v == n {
            times += 1;
        }
    }

    times
}
