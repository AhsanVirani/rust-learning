fn main() {
    let arr: [u8; 3] = [1, 2, 3];

    let v: Vec<u8> = Vec::from(arr);
    is_vec(v);

    let v: Vec<u8> = vec![1, 2, 3];
    is_vec(v.clone());

    let mut v1: Vec<u8> = Vec::new();
    is_vec(v1.clone());

    for i in &v {
        v1.push(*i);
    }

    assert_eq!(v, v1);

    v1.pop(); // removes last elem
    v1.push(3); // pushing to the last elem

    let mut v2: Vec<u8> = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    // iterators can be collected into vectors
    let v3: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v3, vec![0; 10]);

    println!("SUccess");
}

fn is_vec(v: Vec<u8>) {

}