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
        println!("In line {:?}", &line);
        let line: Vec<&str> = line.split("   ").collect();
        let left = line[0];
        let right = line[1];

        println!("first n{:?} second n{:?}", &left, &right);

        first.push(left.parse().unwrap());
        second.push(right.parse().unwrap());
    }

    first.sort();
    second.sort();

    let mut sum = 0;
    println!("{:?}", first.len());

    for i in 0..first.len() {
        let left = first[i];
        let right = second[i];

        let difference = right.abs_diff(left);

        sum = sum + difference;
        println!("The difference on the {} is {}", i, difference);
    }

    println!("The sum of all differences is {}", sum);
}
