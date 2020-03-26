struct Cashier {
    n: i32,
    count: i32,
    discount: i32,
    p2p: std::collections::HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        Self {
            n,
            count: 0,
            discount,
            p2p: products.into_iter().zip(prices.into_iter()).collect()
        }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.count += 1;
        let mut bill: f64 = product.iter().zip(amount.iter()).map(|(p, a)| {
            *self.p2p.get(p).unwrap() as f64 * *a as f64
        }).sum();
        if self.count == self.n {
            self.count = 0;
            bill = (bill * (100 - self.discount) as f64).round() / 100.0;
        }
        bill
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */