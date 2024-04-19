use sp_arithmetic::FixedU128;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod compounding;

#[wasm_bindgen]
pub fn compound(principal: u64, rate_per_second: f64, period: u64) -> Option<u64> {
    compounding::compound(
        principal as u128,
        &FixedU128::from_float(rate_per_second),
        period,
    )
    .map(|res| res as u64)
    .ok()
}

#[wasm_bindgen]
pub fn calculate_new_debt(
    last_fee_update_time: u64,
    now: u64,
    collateral_interest_coefficient: f64,
    cdp_interest_coefficient: f64,
    stability_fee_rate: f64,
    cdp_debt: f64,
) -> Option<String> {
    compounding::calculate_new_debt(
        last_fee_update_time,
        now,
        &FixedU128::from_float(collateral_interest_coefficient),
        &FixedU128::from_float(cdp_interest_coefficient),
        &FixedU128::from_float(stability_fee_rate),
        FixedU128::from_float(cdp_debt).into_inner(),
    )
        .map(|res| res.to_string() )
        .ok()
}

#[test]
fn test_calculate_new_debt() {
    let last_fee_update_time = 1713444654000;
    let now = 1713463477630u64;
    let collateral_interest_coefficient = 1.001518439410635438;
    let cdp_interest_coefficient = 1.00680734730988847;
    let stability_fee_rate = 0.000000000315313331;
    let debt = 150.28432181730841087;

    assert_eq!(
        calculate_new_debt(
            last_fee_update_time,
            now,
            collateral_interest_coefficient,
            cdp_interest_coefficient,
            stability_fee_rate,
            debt,
        ).unwrap(),
        "150384797507575011211"
    );
}
