import { useEffect, useState } from 'react';
import { add, bfs_shortest_path, Graph } from '@imreangelo/pathfinder-wasm';
import GraphCanvas from './components/graph-canvas';
import './App.css'

function App() {
	// Simple test that WASM build is working
	console.log("2 + 3 =", add(2, 3));

	const [graph, setGraph] = useState<any>(null);
	const [highlight, setHighlight] = useState<number[]>([]);

	useEffect(() => {
		(async () => {
			const g = Graph.new_grid(4, 4);
			setGraph(g);

			const path = Array.from(bfs_shortest_path(g, 0, 15));
  			console.log("Shortest path:", path);
			setHighlight(path);
		})();
	}, []);

	return (
		<div className="flex flex-col items-center gap-2">
			<h2>Graph Visualization</h2>
			{graph && <GraphCanvas graph={graph} highlight={highlight} />}
		</div>
	)
}

export default App
