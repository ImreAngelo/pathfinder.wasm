import React, { useMemo } from "react";

export type GridNode = {
	id: number;
	x: number;
	y: number;
	walkable: boolean;
};

type Highlight = GridNode | number;

export type NodeGridProps = {
	/** All nodes to render */
	nodes: GridNode[];

	/**
	 * Nodes (or node ids) to render in a third color.
	 * Highlight color wins over walkable/blocked.
	 */
	highlights?: Highlight[];

	/** Cell size in pixels (each node is 1x1 cell) */
	cellSize?: number;

	/** Optional click handler */
	onCellClick?: (node: GridNode) => void;

	/** Optional colors */
	colors?: {
		walkable: string;
		blocked: string;
		highlighted: string;
		border: string;
		background: string;
	};

	className?: string;
	style?: React.CSSProperties;
};

export function NodeGrid({
	nodes,
	highlights = [],
	cellSize = 14,
	onCellClick,
	colors = {
		walkable: "#f59e0b",     // light gray
		blocked: "#111827",      // near-black
		highlighted: "green",  // amber
		border: "#9ca3af",       // gray
		background: "#ffffff",
	},
	className,
	style,
}: NodeGridProps) {
	const highlightIds = useMemo(() => {
		const s = new Set<number>();
		for (const h of highlights) s.add(typeof h === "number" ? h : h.id);
		return s;
	}, [highlights]);

	const { width, height } = useMemo(() => {
		let maxX = 0;
		let maxY = 0;
		for (const n of nodes) {
			if (n.x > maxX) maxX = n.x;
			if (n.y > maxY) maxY = n.y;
		}
		return { width: maxX + 1, height: maxY + 1 };
	}, [nodes]);

	return (
		<div
			className={className}
			style={{
				display: "grid",
				gridTemplateColumns: `repeat(${width}, ${cellSize}px)`,
				gridTemplateRows: `repeat(${height}, ${cellSize}px)`,
				background: colors.background,
				lineHeight: 0,
				...style,
			}}
		>
			{nodes.map((n) => {
				const isHighlighted = highlightIds.has(n.id);
				const bg = isHighlighted
					? colors.highlighted
					: n.walkable
						? colors.walkable
						: colors.blocked;

				return (
					<div
						key={n.id}
						title={`id=${n.id} (${n.x},${n.y}) walkable=${n.walkable}`}
						onClick={onCellClick ? () => onCellClick(n) : undefined}
						style={{
							width: cellSize,
							height: cellSize,
							background: bg,
							border: `1px solid ${colors.border}`,
							boxSizing: "border-box",

							// place by coordinates (so ordering of `nodes` doesn't matter)
							gridColumnStart: n.x + 1,
							gridRowStart: n.y + 1,

							cursor: onCellClick ? "pointer" : "default",
						}}
					/>
				);
			})}
		</div>
	);
}
