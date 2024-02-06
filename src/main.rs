// Uniswap pool modeling given x*y=k invariant

use std::collections::HashMap;

// Define a structure to represent the liquidity pool
struct UniswapV2Pool {
    base_token_reserve: f64, // WETH, USDC, etc...
    token_a_reserve: f64,
    k: f64, // invariant
    price: f64,
    total_lp_tokens: f64,
    user_lp_tokens: HashMap<String, f64>, // user name, LP tokens
}

impl UniswapV2Pool {
    // Constructor to initialize the pool with initial reserves
    fn new() -> Self {
        UniswapV2Pool {
            base_token_reserve: 0.0,
            token_a_reserve: 0.0,
            k: 0.0,
            price: 0.0,
            total_lp_tokens: 0.0,
            user_lp_tokens: HashMap::new(),
        }
    }

    // Function to provide liquidity to the pool
    fn add_liquidity(&mut self, amount_base: f64, amount_a: f64, user_name: String) {
        let user: String = user_name.clone();
        // if this is not the first time liquidity is added, ensure the price does not change
        if self.base_token_reserve as u64 > 0 && self.token_a_reserve as u64 > 0 {
            assert_eq!(
                self.price,
                amount_base / amount_a,
                "Liquidity invalid. Price changed."
            );
        }

        // Update the reserves
        self.base_token_reserve += amount_base;
        self.token_a_reserve += amount_a;
        self.k = self.base_token_reserve * self.token_a_reserve;
        self.price = self.base_token_reserve / self.token_a_reserve;

        // Calculate the LP tokens to be issued to the user
        let user_lp_tokens = (amount_base * amount_a).sqrt();
        self.total_lp_tokens += user_lp_tokens;
        self.user_lp_tokens.insert(user.clone(), user_lp_tokens);

        println!("=== {} Adds liquidity ===", user);

        println!(
            "New reserves: Base={}, A={}, Price={}",
            self.base_token_reserve, self.token_a_reserve, self.price
        );

        println!(
            "{} received {} LP tokens. Total LP tokens={}\n",
            user, user_lp_tokens, self.total_lp_tokens
        );
    }

    fn remove_liquidity(&mut self, user_name: String) {
        let user: String = user_name.clone();
        if let Some(user_lp_tokens) = self.user_lp_tokens.get(&user) {
            // Calculate the amount of base token and token A to be returned to the user
            let amount_base = (self.base_token_reserve * user_lp_tokens) / self.total_lp_tokens;
            let amount_a = (self.token_a_reserve * user_lp_tokens) / self.total_lp_tokens;

            // Update the reserves
            self.base_token_reserve -= amount_base;
            self.token_a_reserve -= amount_a;
            self.k = self.base_token_reserve * self.token_a_reserve;
            self.price = self.base_token_reserve / self.token_a_reserve;

            println!("=== {} Removes liquidity ===", user);

            println!("{} received Base={} and A={}", user, amount_base, amount_a);

            println!(
                "New reserves: Base={}, A={}, Price={}",
                self.base_token_reserve, self.token_a_reserve, self.price
            );
        } else {
            println!("User {} not found in the LP tokens list.", user);
        }

        // Remove the user from the LP tokens list
        if let Some(user_lp_tokens) = self.user_lp_tokens.get(&user) {
            self.total_lp_tokens -= user_lp_tokens;
            println!(
                "{} removed {} LP tokens. Total LP tokens={}\n",
                user, user_lp_tokens, self.total_lp_tokens
            );

            self.user_lp_tokens.remove(&user);
        }
    }

    // Function to swap tokens in the pool
    fn swap_tokens(&mut self, amount_in: f64, token_out: &str) {
        // Determine which token to swap
        if token_out == "token_base" {
            // Calculate the amount of base token given the amount of token A to be swapped
            let amount_out: f64 =
                self.base_token_reserve - (self.k / (self.token_a_reserve + amount_in));

            // Update the reserves
            self.base_token_reserve -= amount_out;
            self.token_a_reserve += amount_in;
            self.price = self.base_token_reserve / self.token_a_reserve;
        } else {
            // Calculate the amount of token A given the amount of base token to be swapped
            let amount_out: f64 =
                self.token_a_reserve - (self.k / (self.base_token_reserve + amount_in));

            // Update the reserves
            self.token_a_reserve -= amount_out;
            self.base_token_reserve += amount_in;
            self.price = self.base_token_reserve / self.token_a_reserve;
        }

        println!("=== Token swap ===");

        println!(
            "{} Tokens swapped for {}. New reserves: Base={}, A={}, New price={}\n",
            amount_in, token_out, self.base_token_reserve, self.token_a_reserve, self.price
        );
    }

    // Function to get the current reserves of the pool
    fn get_reserves(&self) -> (f64, f64) {
        (self.base_token_reserve, self.token_a_reserve)
    }

    // Function to get the current price of the pool
    fn get_price(&self) -> f64 {
        self.price
    }
}

fn main() {
    // Initialize the Uniswap V2 pool with initial reserves
    let mut uniswap_pool: UniswapV2Pool = UniswapV2Pool::new();

    // Add more liquidity to the pool
    uniswap_pool.add_liquidity(5000.0, 10000.0, "Alice".to_string());

    // Add more liquidity to the pool
    uniswap_pool.add_liquidity(5000.0, 10000.0, "Bob".to_string());

    // Swap tokens in the pool
    uniswap_pool.swap_tokens(50.0, "token_base");

    // Alice removes liquidity from the pool
    uniswap_pool.remove_liquidity("Alice".to_string());

    println!("======== Pool status ========");

    // Get the current reserves of the pool
    let (base_token_reserve, token_a_reserve) = uniswap_pool.get_reserves();
    println!(
        "Current reserves: Base={}, a={}",
        base_token_reserve, token_a_reserve
    );

    // Get the current price of the pool
    let price = uniswap_pool.get_price();
    println!("Current price: {}", price);
}
