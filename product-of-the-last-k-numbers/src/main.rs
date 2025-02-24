struct ProductOfNumbers {
    prefix: Vec<i32>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { prefix: vec![1] }
    }

    fn add(&mut self, num: i32) {
        match num {
            0 => self.prefix = vec![1], // Reset on zero
            _ => self.prefix.push(self.prefix.last().unwrap() * num),
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.prefix.len();
        if k as usize >= n {
            0 // Return 0 if k exceeds valid range (due to reset)
        } else {
            self.prefix[n - 1] / self.prefix[n - 1 - k as usize]
        }
    }
}

fn main() {
    let mut product = ProductOfNumbers::new();

    for i in 0..=5 {
        product.add(i);
    }

    println!("{:?}", product.get_product(2));
}
