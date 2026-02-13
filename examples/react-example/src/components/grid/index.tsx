import React, { useMemo } from "react";

export type Node = {
	id: number;
	x: number;
	y: number;
	cost: number;
};

type Highlight = Node | number;

export type NodeGridProps = {
	nodes: Node[];
	highlights?: Highlight[];
	cellSize?: number;
	onCellClick?: (node: Node) => void;
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
		walkable: "#f59e0b",
		blocked: "#111827",
		highlighted: "#a80780",
		border: "#1b1b1b",
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
				...style,
			}}
		>
			{nodes.map((n) => {
				const isHighlighted = highlightIds.has(n.id);
				const bg = isHighlighted
					? colors.highlighted
					: n.cost > 0
						? colors.walkable
						: colors.blocked;

				const coordColor = n.cost <= 0 && !isHighlighted ? "#f9fafb" : "#111827";

				return (
					<div
						key={n.id}
						title={`id=${n.id} (${n.x},${n.y}) cost=${n.cost}`}
						onClick={onCellClick ? () => onCellClick(n) : undefined}
						style={{
							width: cellSize,
							height: cellSize,
							background: bg,
							border: `1px solid ${colors.border}`,
							boxSizing: "border-box",
							position: "relative",

							// place by coordinates (ordering of `nodes` doesn't matter)
							gridColumnStart: n.x + 1,
							gridRowStart: n.y + 1,

							cursor: onCellClick ? "pointer" : "default",
							userSelect: "none",
						}}
					>
						<span
							style={{
								position: "absolute",
								left: 2,
								bottom: 1,
								fontSize: Math.max(8, Math.floor(cellSize * 0.15)),
								lineHeight: 1,
								fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace",
								color: coordColor,
								pointerEvents: "none",
								whiteSpace: "nowrap",
							}}
						>
							{n.x},{n.y}
						</span>
					</div>
				);
			})}
		</div>
	);
}
