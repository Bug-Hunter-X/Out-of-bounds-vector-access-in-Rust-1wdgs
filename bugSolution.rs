fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    if vec.len() > 2 {
        let val = vec[2];
        println!("Value: {}", val);
    } else {
        println!("Vector does not contain an element at index 2");
    }
} 