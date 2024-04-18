use rand::Rng;
use hex::encode;
struct LiquidityPool {
    e: i32,
    t: i32,
    l: i32,
    k: i32,
    p: f32
}

struct Account {
    address: String,
    e_balance: i32,
    t_balance: i32
}

impl Account {
    fn new(e_balance: i32, t_balance: i32) -> Self {
        let mut rng = rand::thread_rng();
        let address: [u8; 20] = rng.gen();
        let hex_address = encode(address);
        Account {address: hex_address, e_balance, t_balance}
    }

    fn update_e_balance(&mut self, delta_e: i32) {
        self.e_balance = self.e_balance + delta_e;
    }

    fn update_t_balance(&mut self, delta_t: i32) {
        self.t_balance = self.t_balance + delta_t;
    }

    fn swap_from_eth(&mut self, pool: &mut LiquidityPool, delta_e: i32) {
        let delta_t = pool.eth_to_token(delta_e);
        self.update_e_balance(-delta_e);
        self.update_t_balance(delta_t);
    }

    fn swap_exact(&mut self, pool: &mut LiquidityPool, delta_t: i32) {
        let delta_e = pool.eth_to_token_exact(delta_t);
        self.update_e_balance(-delta_e);
        self.update_t_balance(-delta_t);
    }

    fn swap_from_token(&mut self, pool: &mut LiquidityPool, delta_t: i32) {
        let delta_e = pool.token_to_eth(delta_t);
        self.update_e_balance(delta_e);
        self.update_t_balance(-delta_t);
    }
}


impl LiquidityPool {
    fn new(e: i32, t: i32, l: i32, p: f32) -> Self {
        let k: i32 = e * t;
        LiquidityPool {e, t, l, k, p}
    }

    fn add_liquidity(&mut self, delta_e: i32) {

        if delta_e <= 0 {
            panic!("delta_e must be greater than 0");
        }

        let a: f32 = delta_e as f32 / self.e as f32;
        self.e = self.e + delta_e;

        // INFO: apply approximation scheme
        self.t = ((1.0 + a) * self.t as f32) as i32 + 1;
        self.l = ((1.0 + a) * self.l as f32) as i32;
        self.k = self.e * self.t;
    }

    fn remove_liquidity(&mut self, delta_l: i32) {
        if delta_l <= 0 {
            panic!("delta_l must be greater than 0");
        }

        let a: f32 = delta_l as f32 / self.l as f32;
        self.l = self.l - delta_l;

        // INFO: apply approximation scheme
        self.t = ((1.0 - a) * self.t as f32) as i32;
        self.e = ((1.0 - a) * self.e as f32) as i32;
        self.k = self.e * self.t;
    }

    // INFO: calculate how many tokens you will get for a given amount of input tokens
    // xy = (x + dx)(y - dy)
    // a = dx / x; b= dy / y; r = 1 - p;
    // x' = x + dx; y' = y - dy;
    fn get_input_price(delta_x: i32, x: i32, y: i32, p: f32) -> i32 {
        let r: f32 = 1.0 - p;
        let a: f32 = delta_x as f32 / x as f32;
        let delta_y: i32 = ((a as f32 * r as f32 / (1 as f32 + a as f32 * r)) * y as f32) as i32;
        return delta_y;
    }

    fn get_output_price(delta_y: i32, x: i32, y: i32, p: f32) -> i32 {
        let r: f32 = 1.0 - p;
        let b: f32 = delta_y as f32 / y as f32;
        let delta_x: i32 = ((b as f32 / (1 as f32 - b as f32)) / r * x as f32) as i32;
        return delta_x;
    }

    fn eth_to_token(&mut self, delta_x: i32) -> i32 {
        let deleta_y: i32 = LiquidityPool::get_input_price(delta_x, self.e, self.t, self.p);
        self.e = self.e + delta_x;
        self.t = self.t - deleta_y;
        deleta_y
    }

    fn eth_to_token_exact(&mut self, delta_y: i32) -> i32 {
        let delta_x: i32 = LiquidityPool::get_output_price(delta_y, self.e, self.t, self.p);
        self.e = self.e + delta_x;
        self.t = self.t - delta_y;
        delta_x
    }

    fn token_to_eth(&mut self, delta_y: i32) -> i32 {
        let delta_x: i32 = LiquidityPool::get_input_price(delta_y, self.t, self.e, self.p);
        self.e = self.e - delta_x;
        self.t = self.t + delta_y;
        delta_x
    }

    fn token_to_eth_exact(&mut self, delta_x: i32) -> i32 {
        let delta_y: i32 = LiquidityPool::get_output_price(delta_x, self.t, self.e, self.p);
        self.e = self.e - delta_x;
        self.t = self.t + delta_y;
        delta_y
    }
}

fn main() {
    let mut pool = LiquidityPool::new(100, 100, 100, 0.003);
    let mut account = Account::new(100, 100);
    let mut account_sub = Account::new(100, 100);
    println!("e: {}, t: {}, l: {}, k: {}, price: {}", pool.e, pool.t, pool.l, pool.k, pool.t as f32 / pool.e as f32);
    pool.add_liquidity(100);
    println!("e: {}, t: {}, l: {}, k: {}", pool.e, pool.t, pool.l, pool.k);
    pool.add_liquidity(200);
    println!("e: {}, t: {}, l: {}, k: {}", pool.e, pool.t, pool.l, pool.k);
    account.swap_from_eth(&mut pool, 100);
    println!("address: {}, e_balance: {}, t_balance: {}", account.address, account.e_balance, account.t_balance);
    println!("e: {}, t: {}, l: {}, k: {}, price: {}", pool.e, pool.t, pool.l, pool.k, pool.t as f32 / pool.e as f32);
    account_sub.swap_from_token(&mut pool, 100);
    println!("address: {}, e_balance: {}, t_balance: {}", account_sub.address, account_sub.e_balance, account_sub.t_balance);
    println!("e: {}, t: {}, l: {}, k: {}, price: {}", pool.e, pool.t, pool.l, pool.k, pool.t as f32 / pool.e as f32);
}