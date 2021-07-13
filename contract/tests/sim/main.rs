pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk::serde_json::json;
use near_sdk_sim::{call, view, deploy, init_simulator, to_yocto, ContractAccount};
use rust_counter_tutorial::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../out/main.wasm",
}

pub const DEFAULT_GAS: u64 = 300_000_000_000_000;

#[test]
fn simulate_increment() {
    let root = init_simulator(None);
    let _user1 = root.create_user("foo.root".to_string(), to_yocto("1000"));

    // Deploy the compiled Wasm bytes
    let counter: ContractAccount<CounterContract> = deploy!(
        contract: CounterContract,
        contract_id: "counter".to_string(),
        bytes: &COUNTER_BYTES,
        signer_account: root
    );

    // Get number on account that hasn't incremented or decremented
    let mut current_num: i8 = view!(
        counter.get_num()
    ).unwrap_json();
    println!("Number before: {}", &current_num);
    assert_eq!(&current_num, &0, "Initial number should be zero.");

    // Call the increment function
    call!(
        root,
        counter.increment()
    ).assert_success();

    current_num = view!(
        counter.get_num()
    ).unwrap_json();
    println!("Number after first increment: {}", &current_num);
    assert_eq!(&current_num, &1, "After incrementing, the number should be one.");

    // Now use the non-macro approach to increment the number.
    root.call(
        counter.account_id(),
        "increment",
        &json!({})
            .to_string()
            .into_bytes(), // 0.06 Ⓝ
        DEFAULT_GAS,
        0, // attached deposit
    ).assert_success();

    // Similarly, use the non-macro approach to check the value.
    current_num = root.view(
        counter.account_id(),
        "get_num",
        &json!({})
            .to_string()
            .into_bytes(),
    ).unwrap_json();
    println!("Number after second increment: {}", &current_num);
    assert_eq!(&current_num, &2, "After incrementing twice, the number should be two.");
}
