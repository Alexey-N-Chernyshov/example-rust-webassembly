use sp_arithmetic::traits::One;
use sp_arithmetic::traits::{
    CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, EnsureAdd, EnsureMul, Saturating,
};
use sp_arithmetic::{ArithmeticError, FixedU128};

pub type Balance = u128;

/// Per second compounding formula
///
/// Interest is accrued over number of periods
/// `A = P * (1 + period_rate) ^ period`, where:
/// - principal - (P) - initial balance
/// - rate_per_second - (period_rate) - rate per second, where
/// rate_secondly = (1 + rate_annual)^(1/seconds_per_year) - 1
/// - period - time passed in seconds
/// Returns (A) - the new balance with interest over time
pub fn compound(
    principal: Balance,
    rate_per_second: &FixedU128,
    period: u64,
) -> Result<Balance, ArithmeticError> {
    let res = FixedU128::from_inner(principal)
        .ensure_mul(
            FixedU128::from(1)
                .ensure_add(*rate_per_second)?
                .saturating_pow(period as usize),
        )?
        .into_inner();
    if res == Balance::MAX {
        Err(ArithmeticError::Overflow)
    } else {
        Ok(res)
    }
}

pub fn calculate_collateral_interest_coefficient(
    last_fee_update_time: u64,
    now: u64,
    collateral_interest_coefficient: &FixedU128,
    stability_fee_rate: &FixedU128,
) -> Result<Balance, ArithmeticError> {
    let time_passed = now
        .checked_sub(last_fee_update_time)
        .ok_or(ArithmeticError::Underflow)?;
    compound(
        collateral_interest_coefficient.into_inner(),
        stability_fee_rate,
        time_passed,
    )
}

pub fn calculate_new_debt(
    last_fee_update_time: u64,
    now: u64,
    collateral_interest_coefficient: &FixedU128,
    cdp_interest_coefficient: &FixedU128,
    stability_fee_rate: &FixedU128,
    cdp_debt: Balance,
) -> Result<Balance, ArithmeticError> {
    let new_collateral_interest_coefficient = FixedU128::from_inner(calculate_collateral_interest_coefficient(
        last_fee_update_time,
        now,
        collateral_interest_coefficient,
        stability_fee_rate,
    )?);
    let interest_percent = new_collateral_interest_coefficient
        .checked_sub(cdp_interest_coefficient)
        .ok_or(ArithmeticError::Underflow)?
        .checked_div(cdp_interest_coefficient)
        .ok_or(ArithmeticError::DivisionByZero)?;
    Ok(FixedU128::from_inner(cdp_debt)
        .checked_mul(
            &interest_percent
                .checked_add(&FixedU128::one())
                .ok_or(ArithmeticError::Overflow)?,
        )
        .ok_or(ArithmeticError::Underflow)?
        .into_inner())
}

#[test]
fn tests_compound_zero_rate() {
    let initial_balance = 10000000000000000000000;
    let rate = FixedU128::from(0);
    // 1 year in milliseconds
    let time = 31556952000;
    // balance shall not change
    assert_eq!(
        compound(initial_balance, &rate, time).unwrap(),
        initial_balance
    );
}

#[test]
fn test_compound_zero_principal() {
    let initial_balance = 0;
    let rate = FixedU128::from(11);
    // 1 year in milliseconds
    let time = 31556952000;
    // shall not change
    assert_eq!(
        compound(initial_balance, &rate, time).unwrap(),
        initial_balance
    );
}

#[test]
fn test_compound_0_period() {
    // per second rate
    let rate = FixedU128::from_float(0.15);
    let initial_balance = 1000000000000000000000;
    // 1 second
    let time = 0;
    assert_eq!(
        compound(initial_balance, &rate, time).unwrap(),
        1000000000000000000000
    );
}

#[test]
fn test_compound_1_period() {
    // per second rate
    let rate = FixedU128::from_float(0.15);
    let initial_balance = 1000000000000000000000;
    // 1 second
    let time = 1;
    assert_eq!(
        compound(initial_balance, &rate, time).unwrap(),
        1150000000000000000000
    );
}

#[test]
fn test_compound_2_periods() {
    // per second rate
    let rate = FixedU128::from_float(0.1);
    let initial_balance = 1000000000000000000;
    // 1 second
    let time = 2;
    assert_eq!(
        compound(initial_balance, &rate, time).unwrap(),
        1210000000000000000
    );
}

#[test]
fn test_compound_overflow() {
    // per second rate
    let rate = FixedU128::from_float(0.15);
    let initial_balance = Balance::MAX;
    // 1 second
    let time = 1;
    assert_eq!(
        compound(initial_balance, &rate, time),
        Err(ArithmeticError::Overflow)
    );
}
