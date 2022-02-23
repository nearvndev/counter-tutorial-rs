// Counter smart contract workshop
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
struct Counter {
    value: u8,
    new_value: u8
}

#[near_bindgen]
impl Counter {

    #[init]
    pub fn new() -> Self {
        Counter::default()
    }

    pub fn get_num(&self) -> u8 {
        self.value
    }

    pub fn get_new_num(&self) -> u8 {
        self.new_value
    }

    #[payable]
    pub fn set_num(&mut self, new_value: u8) {
        self.value = new_value;
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }
}

