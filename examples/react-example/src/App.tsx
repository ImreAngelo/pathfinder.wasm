import { useState } from 'react'
import { add, bfs, Graph } from '@imreangelo/pathfinder-wasm';
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
	const [count, setCount] = useState(0)

	console.log("2 + 3 =", add(2, 3));

	// Create a 4×4 grid graph
	const g = Graph.new_grid(4, 4);
	console.log("Graph size:", g.size());

	// Do BFS from the top-left cell
	console.log("BFS order:", g.bfs(0));

	// Get neighbors of node 0
	console.log("Neighbors of 0:", g.neighbors(0));

	return (
		<>
			<div>
				<a href="https://vite.dev" target="_blank">
					<img src={viteLogo} className="logo" alt="Vite logo" />
				</a>
				<a href="https://react.dev" target="_blank">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
			</div>
			<h1>Vite + React</h1>
			<div className="card">
				<button onClick={() => setCount((count) => count + 1)}>
					count is {count}
				</button>
				<p>
					Edit <code>src/App.tsx</code> and save to test HMR
				</p>
			</div>
			<p className="read-the-docs">
				Click on the Vite and React logos to learn more
			</p>
		</>
	)
}

export default App
