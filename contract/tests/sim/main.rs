pub use near_sdk::json_types::{Base64VecU8, ValidAccountId, WrappedDuration, U64};
use near_sdk_sim::{call, view, deploy, init_simulator, to_yocto, ContractAccount};
use rust_counter_tutorial::CounterContract;

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    COUNTER_BYTES => "../out/main.wasm",
}

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
    println!("Number after: {}", &current_num);
    assert_eq!(&current_num, &1, "After adding, the number should be one.");
}
