fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 10, 22, 444, 56676];
    adder(&numbers);
    println!("{:?}", adder(&numbers).unwrap());
}


fn adder(numbers: &[u32]) -> Option<u32> {
    let sum = numbers.iter().sum();
    if sum > 4294967294 {
        return None;
    } else {
        return Some(sum);
    }
}
