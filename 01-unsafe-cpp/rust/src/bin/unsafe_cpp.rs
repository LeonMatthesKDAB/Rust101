fn main() {
    let mut vec = vec![1,2,3,4,5,6,7,8,9];
    for value in &vec {
        if vec.len() > 20 {
            break;
        }

        vec.push(*value);
    }

    vec.retain(|&i| i % 2 != 0);

    for value in vec {
        println!("Value: " + value);
    }
}
