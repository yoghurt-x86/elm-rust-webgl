/* tslint:disable */
/* eslint-disable */
/**
*/
export class Client {
  free(): void;
/**
* @param {Element} element
* @param {any} global
*/
  constructor(element: Element, global: any);
/**
* @param {number} time
* @param {number} canvas_height
* @param {number} canvas_width
* @param {Set<any>} held_keys
* @param {Movement} mouse_movement
* @param {boolean} viewport_active
* @param {Array<any>} messages
* @returns {OutMsg}
*/
  update(time: number, canvas_height: number, canvas_width: number, held_keys: Set<any>, mouse_movement: Movement, viewport_active: boolean, messages: Array<any>): OutMsg;
/**
*/
  render(): void;
}
/**
*/
export class Color {
  free(): void;
/**
*/
  b: number;
/**
*/
  g: number;
/**
*/
  r: number;
}
/**
*/
export class Global {
  free(): void;
/**
*/
  ambient_light_color: Color;
/**
*/
  env_light_color: Color;
/**
*/
  fov: number;
/**
*/
  gradient1: Color;
/**
*/
  gradient2: Color;
}
/**
*/
export class Movement {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} x
* @param {number} y
* @returns {Movement}
*/
  static from(x: number, y: number): Movement;
/**
*/
  x: number;
/**
*/
  y: number;
}
/**
*/
export class OutMsg {
  free(): void;
/**
*/
  fps: number;
/**
*/
  time: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_color_free: (a: number) => void;
  readonly __wbg_get_color_r: (a: number) => number;
  readonly __wbg_set_color_r: (a: number, b: number) => void;
  readonly __wbg_get_color_g: (a: number) => number;
  readonly __wbg_set_color_g: (a: number, b: number) => void;
  readonly __wbg_get_color_b: (a: number) => number;
  readonly __wbg_set_color_b: (a: number, b: number) => void;
  readonly __wbg_get_global_env_light_color: (a: number) => number;
  readonly __wbg_set_global_env_light_color: (a: number, b: number) => void;
  readonly __wbg_get_global_ambient_light_color: (a: number) => number;
  readonly __wbg_set_global_ambient_light_color: (a: number, b: number) => void;
  readonly __wbg_get_global_gradient1: (a: number) => number;
  readonly __wbg_set_global_gradient1: (a: number, b: number) => void;
  readonly __wbg_get_global_gradient2: (a: number) => number;
  readonly __wbg_set_global_gradient2: (a: number, b: number) => void;
  readonly __wbg_client_free: (a: number) => void;
  readonly movement_new: () => number;
  readonly movement_from: (a: number, b: number) => number;
  readonly client_new: (a: number, b: number) => number;
  readonly client_update: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly client_render: (a: number) => void;
  readonly __wbg_set_global_fov: (a: number, b: number) => void;
  readonly __wbg_set_outmsg_time: (a: number, b: number) => void;
  readonly __wbg_set_outmsg_fps: (a: number, b: number) => void;
  readonly __wbg_set_movement_x: (a: number, b: number) => void;
  readonly __wbg_set_movement_y: (a: number, b: number) => void;
  readonly __wbg_get_global_fov: (a: number) => number;
  readonly __wbg_get_outmsg_time: (a: number) => number;
  readonly __wbg_get_outmsg_fps: (a: number) => number;
  readonly __wbg_get_movement_x: (a: number) => number;
  readonly __wbg_get_movement_y: (a: number) => number;
  readonly __wbg_global_free: (a: number) => void;
  readonly __wbg_outmsg_free: (a: number) => void;
  readonly __wbg_movement_free: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h76b1c6c92818c84a: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h46130d05b1f8da29: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
