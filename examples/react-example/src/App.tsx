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
			const g = new Grid(20, 16);

			// Create random obstacles
			const walkable = g.nodes().map(() => Math.random() > 0.35);

			g.batch_set_walkable(Uint8Array.from(walkable));
			
			setGraph(g);
			
			const t0 = performance.now();
			const path = g.bfs(Math.random() * 70, 110 + Math.random() * 70);
			const t1 = performance.now();
			
			setHighlightedIds(Array.from(path ?? []));
				
			console.log(`BFS search found path in ${t1 - t0} milliseconds: `, path);
			
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
