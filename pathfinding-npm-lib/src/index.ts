import init, * as wasm from "../pkg/pathfinding.js";

let initialized = false;

export async function initWasm() {
	if (!initialized) {
		await init("../pkg/pathfinding_bg.wasm");
		initialized = true;
	}
	return wasm;
}

// Convenience wrapper
export async function add(a: number, b: number): Promise<number> {
	const { add } = await initWasm();
	return add(a, b);
}

export async function test(str: string) {
	console.log("Hello!", str);
}