use sp_arithmetic::FixedU128;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod compounding;

mod utils;

#[wasm_bindgen]
pub fn compound(
    principal: u64,
    rate_per_second: f64,
    period: u64
) -> Option<u64> {
    compounding::compound(principal as u128, FixedU128::from_float(rate_per_second), period)
        .map(|res| res as u64)
        .ok()
}

#[test]
fn test_compound() {
    let rate = 0.1;
    let initial_balance = 1000000000000000000;
    // 1 second
    let time = 2;
    assert_eq!(
        compound(initial_balance, rate, time).unwrap(),
        1210000000000000000
    );
}