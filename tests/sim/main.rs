use near_sdk_sim::{init_simulator, UserAccount, ContractAccount, deploy, call, view};
use counter::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "out/counter-tutorial.wasm",
}

const CONTRACT_ID: &str = "counter_contract";

pub fn init() -> (UserAccount, ContractAccount<CounterContract>) {
    let root = init_simulator(None);

    let counter_countract: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: CONTRACT_ID.to_string(),
        bytes: &COUNTER_BYTES,
        signer_account: root,
        init_method: new()
    );

    (root, counter_countract)
}

#[test]
pub fn counter_test() {
    let (root, counter_contract) = init();

    let num: u8 = view!(
        counter_contract.get_num()
    ).unwrap_json();

    assert_eq!(num, 0, "Initial number must be equal zero");

    call!(
        root,
        counter_contract.increment()
    ).assert_success();

    let new_num: u8 = view!(
        counter_contract.get_num()
    ).unwrap_json();

    assert_eq!(new_num, 1, "Increase number from zero to 1");
}