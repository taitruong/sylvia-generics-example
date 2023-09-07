use crate::my_interface_impl::test_utils::MyInterface;
use sylvia::multitest::App;

use crate::contract::multitest_utils::CodeId;

#[test]
fn save() {
    // setup
    let app = App::default();
    let code_id = CodeId::store_code(&app);
    let owner = "owner";
    let contract = code_id.instantiate().call(owner).unwrap();

    contract
        .my_interface_proxy()
        .save_data("foo".to_string())
        .call(owner)
        .unwrap();
    let data = contract.my_interface_proxy().get_data().unwrap();
    assert_eq!(data, "foo".to_string());
}
