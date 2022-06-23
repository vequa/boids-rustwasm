declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	*/
	export function flock_update(): void;
	/**
	* @param {any} js_boid
	*/
	export function flock_add_boid(js_boid: any): void;
	/**
	* @param {any} js_boids
	*/
	export function flock_add_boids(js_boids: any): void;
	/**
	* @param {number} amount
	*/
	export function flock_remove_boids(amount: number): void;
	/**
	* @returns {any}
	*/
	export function flock_get(): any;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly flock_update: () => void;
  readonly flock_add_boid: (a: number) => void;
  readonly flock_add_boids: (a: number) => void;
  readonly flock_remove_boids: (a: number) => void;
  readonly flock_get: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
