import * as wasm from "./kensetsu_bindings_bg.wasm";
import { __wbg_set_wasm } from "./kensetsu_bindings_bg.js";
__wbg_set_wasm(wasm);
export * from "./kensetsu_bindings_bg.js";
