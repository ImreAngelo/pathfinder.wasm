import { useEffect, useState } from 'react';
// import { dijkstra, Graph } from '@imreangelo/pathfinder-wasm';
import GraphCanvas from './components/graph-canvas';
import './App.css'
import { Grid } from '@imreangelo/pathfinder-wasm';

function App() {
	const [graph, setGraph] = useState<any>(null);
	const [highlight, setHighlight] = useState<number[]>([]);

	useEffect(() => {
		(async () => {
			const g = new Grid(10, 10);
			
			const walkable = g.nodes().map(({ id, x, y, walkable }, i) => {
				console.log(`Node ${i}/${id} = (${x}, ${y}) -> ${walkable}`)
				return Math.random() > 0.4;
			})

			console.log(walkable);

			console.log("BFS search found path: ");
			g.bfs(0, 25)?.forEach((v, i) => {
				console.log(`Node ${v}`)
			});

			// const g = Graph.new_grid(4, 4);
			// g.add_weighted_edge(0, 5, 2.0);
			
			// // const path = Array.from(bfs(g, 0, 15));
			// const path = Array.from(dijkstra(g, 0, 15))
			// console.log("Shortest path:", path);
			
			// setGraph(g);
			// setHighlight(path);
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
