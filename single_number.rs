fn main() {
    let input = vec![1, 2, 3, 1, 1, 1, 1, 4, 5, 1, 1, 1, 2, 2, 2, 1, 3];
    let output = find_single_number(input);
    println!("{}", output);
}

fn find_single_number (num_vec : Vec<int>) -> int {
    let mut stack : Vec<int> = Vec::new();
    for num in num_vec.iter() {
        let len = stack.len();
        if len == 0 {
            stack.push(*num);
        } else {
            let last = *stack.get(len-1);
            if last == *num {
                stack.push(*num);
            } else {
                stack.pop();
            }
        }
    }

    return match stack.last() {
        Some(a) => *a,
        None    => -1,
    };
}
