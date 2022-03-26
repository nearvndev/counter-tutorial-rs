// Counter smart contract workshop
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, AccountId, env, BorshStorageKey, PanicOnDefault, BlockHeight};

#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct OldCounter {
    value: u8,
    new_value: u8
}

#[derive(BorshSerialize, BorshDeserialize, Default, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountV1 {
    vote: u8,
    balance: u8
}

#[derive(BorshSerialize, BorshDeserialize, Default, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountV2 {
    vote: u8,
    balance: u8,
    bio: String
}

#[derive(BorshSerialize, BorshDeserialize, Default, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    vote: u8,
    balance: u8,
    bio: String,
    last_change: BlockHeight
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VersionedAccount {
    V1(AccountV1),
    V2(AccountV2),
    Current(Account)
}

impl From<VersionedAccount> for Account {
    fn from(account: VersionedAccount) -> Self {
        match account {
            VersionedAccount::Current(account) => account,
            VersionedAccount::V1(v1) => Account {
                vote: v1.vote,
                balance: v1.balance,
                bio: String::from("migrate bio"),
                last_change: 0
            },
            VersionedAccount::V2(v2) => Account {
                vote: v2.vote,
                balance: v2.balance,
                bio: v2.bio,
                last_change: 0
            }
        }
    }
}

impl From<Account> for VersionedAccount {
    fn from(account: Account) -> Self {
        VersionedAccount::Current(account)
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Counter {
    value: u8,
    new_value: u8,
    accounts: LookupMap<AccountId, VersionedAccount>
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    AccountKey
}

#[near_bindgen]
impl Counter {

    #[init]
    pub fn new() -> Self {
        Counter { 
            value: 0, 
            new_value: 0,
            accounts: LookupMap::new(StorageKey::AccountKey)
        }
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

    pub fn get_account(&self, account_id: AccountId) -> Account {
        let v_account = self.accounts.get(&account_id).unwrap();

        Account::from(v_account)
    }

    pub fn add_account(&mut self) {
        let v_account = self.accounts.get(&env::predecessor_account_id());

        if v_account.is_none() {
            let account = Account::default();
            let v_account = VersionedAccount::from(account);
            self.accounts.insert(&env::predecessor_account_id(), &v_account);
        }
    }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let old_counter: OldCounter = env::state_read().expect("Can not read state");

        Counter { 
            value: old_counter.value, 
            new_value: 0,
            accounts: LookupMap::new(StorageKey::AccountKey)
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::{MockedBlockchain, VMContext};
    use near_sdk::{testing_env};

    use super::*;

    fn john() -> AccountId {
        return "john.testnet".to_string();
    }

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn test_num() {
        let context = get_context(john(), 0);
        testing_env!(context);

        let mut contract = Counter::new();
        let number_before = contract.get_num();
        assert_eq!(0, number_before, "Expected num should be zero");
        contract.set_num(1);

        let number_after = contract.get_num();
        assert_eq!(1, number_after, "Expected two values are the same");

        contract.increment();
        let number_after_increment = contract.get_num();
        assert_eq!(2, number_after_increment, "Expected after increment should be 2");
    }
  }