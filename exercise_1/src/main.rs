use std::io::BufRead;

fn read_vec<T>() -> Vec<T>
where 
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    std::io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn main() {
    println!("Enter elements in the first array (separate by whitespace): ");
    let org_array: Vec<i32> = read_vec::<i32>();

    println!("Enter the elements in the second array (separate by whitespace): ");
    let sub_array: Vec<i32> = read_vec::<i32>();
    
    let mut is_subset = false;

    for i in 0..(org_array.len() - sub_array.len()) {
       if sub_array[0] != org_array[i] {
            continue
       } else {
            let mut count = 0;
            for j in 0..sub_array.len() {
                if org_array[i+j] == sub_array[j] {
                    count += 1;
                } 
            }
            if count == sub_array.len() {
                is_subset = true;
                break;
            }
       }
    }

    if is_subset == true {
        println!("The second array is the subset of the first array.")
    } else {
        println!("The second array is NOT the subset of the first array.")
    }
}

