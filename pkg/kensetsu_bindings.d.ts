/* tslint:disable */
/* eslint-disable */
/**
* @param {bigint} principal
* @param {number} rate_per_second
* @param {bigint} period
* @returns {bigint | undefined}
*/
export function compound(principal: bigint, rate_per_second: number, period: bigint): bigint | undefined;
/**
* @param {bigint} last_fee_update_time
* @param {bigint} now
* @param {number} collateral_interest_coefficient
* @param {number} cdp_interest_coefficient
* @param {number} stability_fee_rate
* @param {number} cdp_debt
* @returns {string | undefined}
*/
export function calculate_new_debt(last_fee_update_time: bigint, now: bigint, collateral_interest_coefficient: number, cdp_interest_coefficient: number, stability_fee_rate: number, cdp_debt: number): string | undefined;
