struct Array<T, const N: usize> {
    data: [T; N]   
}

fn main() {
    let arrays: [Array<i32, 3>; 3] = [
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [4, 5, 7],
        },
        Array {
            data: [8, 10, 11],
        }
    ];

    let floats: [Array<f64, 2>; 3] = [
        Array {
            data: [2.0, 4.3],
        },
        Array {
            data: [66.98, 33.2],
        },
        Array {
            data: [43.92, 31.2],
        },
    ];

    println!("Success");
}