let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let cachedBigInt64Memory0 = null;

function getBigInt64Memory0() {
    if (cachedBigInt64Memory0 === null || cachedBigInt64Memory0.byteLength === 0) {
        cachedBigInt64Memory0 = new BigInt64Array(wasm.memory.buffer);
    }
    return cachedBigInt64Memory0;
}
/**
* @param {bigint} principal
* @param {number} rate_per_second
* @param {bigint} period
* @returns {bigint | undefined}
*/
export function compound(principal, rate_per_second, period) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.compound(retptr, principal, rate_per_second, period);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r2 = getBigInt64Memory0()[retptr / 8 + 1];
        return r0 === 0 ? undefined : BigInt.asUintN(64, r2);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
/**
* @param {bigint} last_fee_update_time
* @param {bigint} now
* @param {number} collateral_interest_coefficient
* @param {number} cdp_interest_coefficient
* @param {number} stability_fee_rate
* @param {number} cdp_debt
* @returns {string | undefined}
*/
export function calculate_new_debt(last_fee_update_time, now, collateral_interest_coefficient, cdp_interest_coefficient, stability_fee_rate, cdp_debt) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.calculate_new_debt(retptr, last_fee_update_time, now, collateral_interest_coefficient, cdp_interest_coefficient, stability_fee_rate, cdp_debt);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        let v1;
        if (r0 !== 0) {
            v1 = getStringFromWasm0(r0, r1).slice();
            wasm.__wbindgen_export_0(r0, r1 * 1, 1);
        }
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

