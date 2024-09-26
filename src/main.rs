use std::io;
fn main() {
    let mut nums = String::new();

    io::stdin().read_line(&mut nums).expect("Can't read");

    let user_input = nums.split(" ");

    let mut result: Vec<i16> = Vec::new();

    for element in user_input {
        let num = element.trim().parse::<i16>().unwrap();
        result.push(num);
    }

    for idx in 1..result.len() {
        let cur = result[idx];
        let mut temp_idx = idx;
        while temp_idx > 0 && cur < result[temp_idx - 1] {
            result[temp_idx] = result[temp_idx - 1];
            temp_idx -= 1;
        }

        result[temp_idx] = cur;
    }
    for element in result {
        print!("{} ", element);
    }
}
