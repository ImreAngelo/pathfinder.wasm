import React, { useEffect, useRef } from "react";

// Type definitions (adjust if you use TypeScript)
type GraphCanvasProps = {
	graph: any;         // wasm Graph instance
	width?: number;     // canvas width
	height?: number;    // canvas height
	highlight?: number[]; // optional list of highlighted nodes (e.g. BFS path)
};

const GraphCanvas: React.FC<GraphCanvasProps> = ({
	graph,
	width = 400,
	height = 400,
	highlight = []
}) => {
	const canvasRef = useRef<HTMLCanvasElement>(null);

	useEffect(() => {
		if (!graph || !canvasRef.current) return;
		const ctx = canvasRef.current.getContext("2d");
		if (!ctx) return;

		const size = graph.size();
		const grid = Math.sqrt(size);
		const spacing = width / (grid + 1);

		ctx.clearRect(0, 0, width, height);
		ctx.lineWidth = 2;
		ctx.strokeStyle = "#aaa";

		// Draw edges
		for (let node = 0; node < size; node++) {
			const neighbors = graph.neighbors(node);
			const x1 = spacing * (node % grid + 1);
			const y1 = spacing * (Math.floor(node / grid) + 1);

			for (const n of neighbors) {
				if (n > node) { // avoid drawing edges twice
					const x2 = spacing * (n % grid + 1);
					const y2 = spacing * (Math.floor(n / grid) + 1);
					ctx.beginPath();
					ctx.moveTo(x1, y1);
					ctx.lineTo(x2, y2);
					ctx.stroke();
				}
			}
		}

		// Draw nodes
		for (let node = 0; node < size; node++) {
			const x = spacing * (node % grid + 1);
			const y = spacing * (Math.floor(node / grid) + 1);

			const isHighlighted = highlight.includes(node);
			ctx.beginPath();
			ctx.arc(x, y, 8, 0, Math.PI * 2);
			ctx.fillStyle = isHighlighted ? "#ff5555" : "#444";
			ctx.fill();
			ctx.strokeStyle = "#fff";
			ctx.stroke();
		}
	}, [graph, width, height, highlight]);

	return <canvas ref={canvasRef} width={width} height={height} />;
};

export default GraphCanvas;
