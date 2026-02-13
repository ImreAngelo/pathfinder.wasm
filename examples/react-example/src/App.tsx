import { useEffect, useState } from 'react';
import { Graph, Grid, Grid2D } from '@imreangelo/pathfinder-wasm';
import { NodeGrid, type Node } from './components/grid';
import './App.css'

function App() {
	const [graph, setGraph] = useState<Grid | null>(null);
	const [nodes, setNodes] = useState<Node[]>([]);
	const [highlightedIds, setHighlightedIds] = useState<number[]>([]);

	useEffect(() => {
		(async () => {
			// const g = new Grid2D(20, 16);
			// console.log("Test:", g.nodes());

			const g = new Grid(20, 16);

			// Create random obstacles
			const walkable = g.nodes().map(() => Math.random() > 0.35);

			g.batch_set_walkable(Uint8Array.from(walkable));
			
			setGraph(g);

			let path = null;
			let t = 0;

			while(path == null) {
				const a = Math.floor(Math.random() * 70);
				const b = Math.floor(Math.random() * 70) + 275;
	
				const t0 = performance.now();
				path = g.bfs(a,b);
				const t1 = performance.now();
				t = t1 - t0;
			}

			console.log(`BFS search found path in ${t} milliseconds: `, path);
			
			setHighlightedIds(Array.from(path ?? []));
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
