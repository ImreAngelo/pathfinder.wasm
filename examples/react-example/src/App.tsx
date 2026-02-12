import { useEffect, useState } from 'react';
// import { dijkstra, Graph } from '@imreangelo/pathfinder-wasm';
// import GraphCanvas from './components/graph-canvas';
import { Grid } from '@imreangelo/pathfinder-wasm';
import { NodeGrid, type GridNode } from './components/grid';
import './App.css'

function App() {
	// const [graph, setGraph] = useState<Grid | null>(null);
	// const [highlight, setHighlight] = useState<number[]>([]);
	const [graph, setGraph] = useState<Grid | null>(null);
	const [nodes, setNodes] = useState<GridNode[]>([]);
	const [highlightedIds, setHighlightedIds] = useState<number[]>([]);

	useEffect(() => {
		(async () => {
			const g = new Grid(10, 10);

			// Create random 
			const walkable = g.nodes().map(() => Math.random() > 0.4)
			
			g.batch_set_walkable(Uint8Array.from(walkable));

			setGraph(g);

			console.log("BFS search found path: ");
			// g.bfs(0, 25)?.forEach(({id, x, y, walkable}, i) => {
			// 	console.log(`Node ${v}`)
			// });

			// If bfs returns Uint32Array of ids:
			// const pathIds = g.bfs(0, 25);
			// setHighlightedIds(Array.from(pathIds ?? []));

			// // Update nodes from the grid (depends on your API)
			setNodes(g.nodes());
		})();
	}, []);

	return (
		<div className="flex flex-col items-center gap-2">
			<h2>Graph Visualization</h2>
			{graph && <NodeGrid nodes={nodes} highlights={highlightedIds} cellSize={48} />}
			{/* {graph && <GraphCanvas graph={graph} highlight={highlight} />} */}
		</div>
	)
}

export default App
