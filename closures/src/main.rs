fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1.iter();
    let v3 = v1.iter().zip(v2).map(|(a, b)| a * b);

    for item in v3{
        println!("{:?}", item)
    }

}
