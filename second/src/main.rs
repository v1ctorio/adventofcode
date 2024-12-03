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
    let mut l2val = 0;

    for level in levels {
        if is_level_safe(&level) {
            valids += 1;
            l2val += 1;
            continue;
        }

        let mut level_is_safe_w_one = false;

        for i in 0..level.len() {
            let mut vec_w_one = level.clone();
            vec_w_one.remove(i);
            if is_level_safe(&vec_w_one) {
                level_is_safe_w_one = true;
            }

            println!("Vec {:?} without {} is {:?}", &level, i, &vec_w_one);
        }

        if level_is_safe_w_one {
            l2val += 1;
        }
    }

    println!("The sum of all valid levels is {}", valids);
    println!("The sum of all valid levels in level 2 is {}", l2val);
}

fn backwards_vector(v: &Vec<u32>) -> Vec<u32> {
    let mut new: Vec<u32> = Vec::new();

    for i in 0..v.len() {
        new.push(v[v.len() - i - 1]);
    }

    new
}

fn is_level_safe(level: &Vec<u32>) -> bool {
    println!("In {:?}", &level);
    let mut is_sorted = false;
    let mut dif_in_range = true;

    let mut sorted = level.clone();
    sorted.sort();
    let bwsorted = backwards_vector(&sorted);

    if level == &sorted || level == &bwsorted {
        is_sorted = true;
    }

    for i in 0..level.len() - 1 {
        let dif = level[i].abs_diff(level[i + 1]);

        if dif < 1 || dif > 3 {
            dif_in_range = false;
            println!("Diff is not in range");
        }
    }

    dif_in_range && is_sorted
}
