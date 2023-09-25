fn main() {
    let a = vec![5, 3];
    let b = vec![0, 2, 3];
    let c = vec![4, 12, 10];
    let d = vec![6, 5, 2];
    // 5: 1,1 3: 1,2
    // 10: 2,1 2: 2,2
    // a11+b21 = a12+b12
    let zip = a.iter().zip(b.iter());
    let mut prefix = 0_i32;
    let mut counter = 0;
    for (x, y) in zip {
        if prefix.eq(&(x + y)) {
            counter += 1;
        }
        prefix = x + y;
    }
    println!("counter : {}", counter);
}
