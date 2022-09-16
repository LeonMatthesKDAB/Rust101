fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut second = vec![];
    for value in &vec {
        if vec.len() + second.len() > 20 {
            break;
        }

        second.push(*value);
    }
    vec.extend(second);

    vec.retain(|&i| i % 2 != 0);

    for value in vec {
        println!("Value: {}", value);
    }
}
