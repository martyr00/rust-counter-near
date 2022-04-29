//! This contract implements simple counter backed by storage on blockchain.
//!
//! The contract provides methods to [increment] / [decrement] counter and
//! get it's current value [get_num] or [reset].
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};


// add the following attributes to prepare your code for serialization and invocation on the blockchain
// More built-in Rust attributes here: https://doc.rust-lang.org/reference/attributes.html#built-in-attributes-index
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: i8,
}

#[near_bindgen]
impl Counter {
    /// Public method: Returns the counter value.
    ///
    /// This must match the type from our struct's 'val' defined above.
    ///
    /// Note, the parameter is `&self` (without being mutable) meaning it doesn't modify state.
    /// In the frontend (/src/main.js) this is added to the "viewMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near view counter.YOU.testnet get_num
    /// ```
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    /// Public method: Increment the counter.
    ///
    /// Note, the parameter is "&mut self" as this function modifies state.
    /// In the frontend (/src/main.js) this is added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet increment --accountId donation.YOU.testnet
    /// ```
    pub fn increment(&mut self) {
        self.val += 1;
        log!("Increased number to {}", self.val);
    }

    /// Public method: Decrement the counter.
    ///
    /// In (/src/main.js) this is also added to the "changeMethods" array
    /// using near-cli we can call this by:
    ///
    /// ```bash
    /// near call counter.YOU.testnet decrement --accountId donation.YOU.testnet
    /// ```
    pub fn decrement(&mut self) {
        self.val -= 1;
        log!("Decreased number to {}", self.val);
    }

    /// Public method - Reset to zero.
    pub fn reset(&mut self) {
        self.val = 0;
        log!("Reset counter to zero");
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be: `cargo test`
 * Note: 'rust-counter-tutorial' comes from cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment() {
        // instantiate a contract variable with the counter at zero
        let mut contract = Counter { val: 0 };
        contract.increment();
        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let mut contract = Counter { val: 0 };
        contract.decrement();
        assert_eq!(-1, contract.get_num());
    }

    #[test]
    fn increment_and_reset() {
        let mut contract = Counter { val: 0 };
        contract.increment();
        contract.reset();
        assert_eq!(0, contract.get_num());
    }

    #[test]
    #[should_panic]
    fn panics_on_overflow() {
        let mut contract = Counter { val: 127 };
        contract.increment();
    }

    #[test]
    #[should_panic]
    fn panics_on_underflow() {
        let mut contract = Counter { val: -128 };
        contract.decrement();
    }
}
