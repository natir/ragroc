/* tslint:disable */
/* eslint-disable */
/**
* @returns {boolean} 
*/
export function set_gc_to(): boolean;
/**
*/
export function finish_load(): void;
/**
* @returns {boolean} 
*/
export function run_generate(): boolean;
/**
* @returns {boolean} 
*/
export function run_revcomp(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly set_gc_to: () => number;
  readonly finish_load: () => void;
  readonly run_generate: () => number;
  readonly run_revcomp: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        