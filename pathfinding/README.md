# Pathfinding in Web Assembly
A package to perform pathfinding without the performance impact of using javascript. 
This package is implemented in Rust and compiled to Web Assembly (WASM), thereby achieving native performance in the browser.

Currently supports dijkstra only. Planned support for A* and JPS, as well as maze graph generation.

## Install
`pnpm add @imreangelo/pathfinder-wasm`

To use with vite/react, add `vite-plugin-wasm` and `vite-plugin-top-level-await` as plugins to your vite.config.ts file.

```js
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

// https://vite.dev/config/
export default defineConfig({
	plugins: [
		react(),
		wasm(),
		topLevelAwait()
	]
})
```

## Usage
Check out the [examples](https://github.com/ImreAngelo/pathfinder.wasm/blob/master/examples/react-example/src/App.tsx).