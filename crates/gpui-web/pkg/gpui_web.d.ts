/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

/**
 * Connect to a WebSocket server and log proto messages to the browser console.
 */
export function connect(url: string): void;

/**
 * Connect to a WebSocket server and call `on_message(json_string)` for each
 * decoded proto Envelope. If `on_message` is undefined, logs to console.
 */
export function connect_with_callback(url: string, on_message: any): void;

/**
 * Launch the GPUI-based file tree view connected to a Zed remote server.
 *
 * This creates a GPUI application, connects to the server via WebSocket,
 * registers proto message handlers, and renders a file tree from the
 * server's worktree state.
 */
export function launch_file_tree(url: string): void;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly connect: (a: number, b: number) => [number, number];
    readonly connect_with_callback: (a: number, b: number, c: any) => [number, number];
    readonly launch_file_tree: (a: number, b: number) => [number, number];
    readonly start: () => void;
    readonly wasm_bindgen_b0c143da9b2857ed___closure__destroy___dyn_core_f4ce2b6cc8c3b44d___ops__function__FnMut__wasm_bindgen_b0c143da9b2857ed___JsValue____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_b0c143da9b2857ed___closure__destroy___dyn_core_f4ce2b6cc8c3b44d___ops__function__FnMut__web_sys_62b7dacf109521b___features__gen_CloseEvent__CloseEvent____Output_______: (a: number, b: number) => void;
    readonly wasm_bindgen_b0c143da9b2857ed___convert__closures_____invoke___wasm_bindgen_b0c143da9b2857ed___JsValue_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_b0c143da9b2857ed___convert__closures_____invoke___web_sys_62b7dacf109521b___features__gen_CloseEvent__CloseEvent_____: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen_b0c143da9b2857ed___convert__closures_____invoke______: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
