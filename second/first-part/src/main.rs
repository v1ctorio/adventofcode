use std::fs;

fn main() {
    let mut levels: Vec<Vec<u32>> = Vec::new();
    let input = fs::read_to_string("input.txt").unwrap();

    let input: Vec<&str> = input.split("\n").collect();
    for line in input {
        if line == "" {
            continue;
        }
        println!("In line {:?}", &line);
        let line: Vec<u32> = line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        levels.push(line);
    }

    let mut valids = 0;

    for level in levels {
        println!("in level {:?}", &level);
        let mut is_sorted = false;
        let mut sorted: Vec<u32> = level.clone();
        sorted.sort();

        let backwards_sorted = backwards_vector(&sorted);

        if level == sorted || level == backwards_sorted {
            is_sorted = true;
            println!("it is sorted");
        }

        let mut dif_is_in_range = true;

        for i in 0..level.len() - 1 {
            let dif = level[i].abs_diff(level[i + 1]);

            //println!("dif is {}", &dif);
            if dif < 1 || dif > 3 {
                dif_is_in_range = false;
                println!("diff is NOT in range")
            }
        }

        if dif_is_in_range && is_sorted {
            valids += 1;
        }
    }

    println!(
        "The backwards of {:?} is {:?}",
        vec![1, 2, 3, 5],
        backwards_vector(&vec![1, 2, 3, 5])
    );
    println!("The sum of all valid levels is {}", valids);
}

fn backwards_vector(v: &Vec<u32>) -> Vec<u32> {
    let mut new: Vec<u32> = Vec::new();

    for i in 0..v.len() {
        new.push(v[v.len() - i - 1]);
    }

    new
}
