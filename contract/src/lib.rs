use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, Promise};

const MIN_DEPOSIT: u128 = 1_000_000_000_000_000_000_000_000;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Counter {
    pub count: i8,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> i8 {
        self.count
    }

    #[payable]
    pub fn increment(&mut self) {
        let amount = env::attached_deposit();
        assert!(amount >= MIN_DEPOSIT, "not enough deposit");

        let money_back: u128 = amount - MIN_DEPOSIT;

        if money_back > 0 {
            Promise::new(env::predecessor_account_id()).transfer(money_back);
        };

        self.count += 1;
        log!(
            "Increased number to {}, moneyback: {}",
            self.count,
            money_back
        );
    }

    pub fn decrement(&mut self) {
        self.count -= 1;
        log!("count = {}", self.count)
    }

    pub fn reset(&mut self) {
        self.count = 0;
        log!("count = {}", self.count)
    }
}
