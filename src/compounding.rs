use sp_arithmetic::traits::{EnsureAdd, EnsureMul, Saturating};
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
    rate_per_second: FixedU128,
    period: u64,
) -> Result<Balance, ArithmeticError> {
    let res = FixedU128::from_inner(principal)
        .ensure_mul(
            FixedU128::from(1)
                .ensure_add(rate_per_second)?
                .saturating_pow(period as usize),
        )?
        .into_inner();
    if res == Balance::MAX {
        Err(ArithmeticError::Overflow)
    } else {
        Ok(res)
    }
}

#[test]
fn tests_compound_zero_rate() {
    let initial_balance = 10000000000000000000000;
    let rate = FixedU128::from(0);
    // 1 year in milliseconds
    let time = 31556952000;
    // balance shall not change
    assert_eq!(
        compound(initial_balance, rate, time).unwrap(),
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
        compound(initial_balance, rate, time).unwrap(),
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
        compound(initial_balance, rate, time).unwrap(),
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
        compound(initial_balance, rate, time).unwrap(),
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
        compound(initial_balance, rate, time).unwrap(),
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
        compound(initial_balance, rate, time),
        Err(ArithmeticError::Overflow)
    );
}
