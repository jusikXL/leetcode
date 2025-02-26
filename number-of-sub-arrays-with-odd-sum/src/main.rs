pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
    const MOD: i32 = 1_000_000_007;

    let mut counter = 0;
    let len = arr.len();

    for i in 0..len {
        let mut sum = 0;

        for j in i..len {
            sum += arr[j];

            if !(sum % 2 == 0) {
                counter += 1;
            }
        }
    }

    counter % MOD
}

fn main() {
    println!("{:?}", num_of_subarrays(vec![2, 4, 6]));
}
