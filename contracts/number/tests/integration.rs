use cosmwasm_std::to_vec;
use cosmwasm_vm::testing::{
    mock_backend, mock_env, write_data_to_mock_env, Contract, MockApi,
    MockInstanceOptions, MockQuerier, MockStorage,
};
use cosmwasm_vm::Instance;
use std::collections::HashMap;
use wasmer_types::{FunctionType, Type};

static CONTRACT: &[u8] = include_bytes!("../target/wasm32-unknown-unknown/release/number.wasm");

fn required_exports() -> Vec<(String, FunctionType)> {
    vec![
        (String::from("stub_add"), ([Type::I32], []).into()),
        (String::from("stub_sub"), ([Type::I32], []).into()),
        (String::from("stub_mul"), ([Type::I32], []).into()),
        (String::from("stub_number"), ([], [Type::I32]).into()),
    ]
}

fn make_number_instance() -> Instance<MockApi, MockStorage, MockQuerier> {
    let options = MockInstanceOptions::default();
    let backend = mock_backend(&[]);
    let mut contract = Contract::from_code(CONTRACT, backend, options).unwrap();
    let instance = contract.generate_instance().unwrap();
    instance
        .env
        .set_serialized_env(&to_vec(&mock_env()).unwrap());

    instance
}

#[test]
fn callable_point_export_works() {
    let options = MockInstanceOptions::default();
    let backend = mock_backend(&[]);
    let contract = Contract::from_code(CONTRACT, backend, options).unwrap();

    let export_function_map: HashMap<_, _> = contract
        .module
        .exports()
        .functions()
        .map(|export| (export.name().to_string(), export.ty().clone()))
        .collect::<Vec<(String, FunctionType)>>()
        .into_iter()
        .collect();

    let required_exports = required_exports();
    for required_export in required_exports {
        match export_function_map.get(&required_export.0) {
            Some(exported_function) => {
                assert_eq!(*exported_function, required_export.1);
            }
            None => panic!("{} is not exported.", required_export.0),
        }
    }
}

#[test]
fn callable_point_add_works() {
    let instance = make_number_instance();

    let serialized_param = to_vec(&10i32).unwrap();
    let param_region_ptr = write_data_to_mock_env(&instance.env, &serialized_param).unwrap();

    let required_exports = required_exports();
    let export_index = 0;
    assert_eq!("stub_add".to_string(), required_exports[export_index].0);

    // Before solving #213, it issues an error.
    // This is because `stub_add` panics without number in deps.storage.
    let call_result = instance
        .call_function_strict(
            &required_exports[export_index].1,
            "stub_add",
            &[param_region_ptr.into()],
        )
        .unwrap_err();
    assert!(call_result.to_string().contains("RuntimeError: unreachable"))
}

#[test]
fn callable_point_sub_works() {
    let instance = make_number_instance();

    let serialized_param = to_vec(&10i32).unwrap();
    let param_region_ptr = write_data_to_mock_env(&instance.env, &serialized_param).unwrap();

    let required_exports = required_exports();
    let export_index = 1;
    assert_eq!("stub_sub".to_string(), required_exports[export_index].0);

    // Before solving #213, it issues an error.
    // This is because `stub_sub` panics without number in deps.storage.
    let call_result = instance
        .call_function_strict(
            &required_exports[export_index].1,
            "stub_sub",
            &[param_region_ptr.into()],
        )
        .unwrap_err();
    assert!(call_result.to_string().contains("RuntimeError: unreachable"))
}

#[test]
fn callable_point_mul_works() {
    let instance = make_number_instance();

    let serialized_param = to_vec(&10i32).unwrap();
    let param_region_ptr = write_data_to_mock_env(&instance.env, &serialized_param).unwrap();

    let required_exports = required_exports();
    let export_index = 2;
    assert_eq!("stub_mul".to_string(), required_exports[export_index].0);

    // Before solving #213, it issues an error.
    // This is because `stub_add` panics without number in deps.storage.
    let call_result = instance
        .call_function_strict(
            &required_exports[export_index].1,
            "stub_mul",
            &[param_region_ptr.into()],
        )
        .unwrap_err();
    assert!(call_result.to_string().contains("RuntimeError: unreachable"))
}

#[test]
fn callable_point_number_works() {
    let instance = make_number_instance();

    let required_exports = required_exports();
    let export_index = 3;
    assert_eq!("stub_number".to_string(), required_exports[export_index].0);
    // Before solving #213, it issues an error.
    // This is because `stub_number` panics without number in deps.storage.
    let call_result = instance
        .call_function_strict(&required_exports[0].1, "stub_number", &[])
        .unwrap_err();
    assert!(call_result.to_string().contains("RuntimeError: unreachable"))
}