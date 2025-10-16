import { useEffect, useState } from 'react';
import { dijkstra, Graph } from '@imreangelo/pathfinder-wasm';
import GraphCanvas from './components/graph-canvas';
import './App.css'

function App() {
	const [graph, setGraph] = useState<any>(null);
	const [highlight, setHighlight] = useState<number[]>([]);

	useEffect(() => {
		(async () => {
			const g = Graph.new_grid(4, 4);
			g.add_weighted_edge(0, 5, 2.0);
			
			// const path = Array.from(bfs(g, 0, 15));
			const path = Array.from(dijkstra(g, 0, 15))
			console.log("Shortest path:", path);
			
			setGraph(g);
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
