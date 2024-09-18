use multiversx_sc::types::BigInt;
use multiversx_my_sc_scenario::api::StaticApi;

#[test]
fn test_big_int_add() {
    let x = BigInt::<StaticApi>::from(2);
    let y = BigInt::<StaticApi>::from(3);
    assert_eq!(x + y, BigInt::<StaticApi>::from(5))
}