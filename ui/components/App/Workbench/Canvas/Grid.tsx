import React, { memo } from "react";
import { calc_node_height, selectQueriedNode, useStore } from "store";

export function Grid() {
  const windowWidth = useStore(state => state.windowWidth);
  const windowHeight = useStore(state => state.windowHeight);
  const canvasX = useStore(state => state.canvasX);
  const canvasY = useStore(state => state.canvasY);
  const zoom = useStore(state => state.canvasZoom);
  const queriedNode = useStore(selectQueriedNode);
  const gap = 32;
  const width = (windowWidth + gap) / zoom;
  const height = (windowHeight + gap) / zoom;
  // FIXME: Zoom transition is not in-sync with the canvas transition.
  const offsetX = canvasX > 0 ? ((canvasX / zoom) % gap - gap) * zoom : ((canvasX / zoom) % gap) * zoom;
  const offsetY = canvasY > 0 ? ((canvasY / zoom) % gap - gap) * zoom : ((canvasY / zoom) % gap) * zoom;
  const strokeWidth = 1;
  const nodeWidth = 256;
  const nodeHeight = queriedNode ? calc_node_height(queriedNode) : 0;
  const light = queriedNode && {
    x: queriedNode.position.x + (canvasX - offsetX) / zoom + nodeWidth / 2,
    y: queriedNode.position.y + (canvasY - offsetY) / zoom + nodeHeight / 2,
    height: nodeHeight,
    width: nodeWidth
  };
  const fill = light ? "url(#gradient)" : "rgba(87, 87, 87, 0.1)";

  return (
    <svg
      style={{
        position: "absolute",
        transform: `translate(${offsetX}px, ${offsetY}px) scale(${zoom})`,
        transformOrigin: "0 0"
      }}
      width={width}
      height={height}
      viewBox={`0 0 ${width} ${height}`}
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <GridLinesMemo
        width={width}
        height={height}
        gap={gap}
        strokeWidth={strokeWidth}
        fill={fill}
      />

      {light && (
        <defs>
          <radialGradient
            id="gradient"
            cx="0"
            cy="0"
            r="1"
            gradientUnits="userSpaceOnUse"
            gradientTransform={`translate(${light.x} ${light.y}) scale(${light.width} ${light.height})`}
          >
            <stop stopColor="#5A1088"/>
            <stop offset="80%" stopColor="#5A1088" stopOpacity="0.2"/>
            <stop offset="100%" stopColor="#575757" stopOpacity="0.1"/>
          </radialGradient>
        </defs>
      )}
    </svg>
  );
}

const GridLinesMemo = memo(GridLines);

interface GridLinesProps {
  width: number,
  height: number,
  gap: number,
  strokeWidth: number,
  fill: string
}

function GridLines({ width, height, gap, strokeWidth, fill }: GridLinesProps) {
  return (
    <React.Fragment>
      {Array(Math.floor(width/32)).fill(0).map((_, i) => (
        <HorizontalLine
          key={i}
          nth={i}
          length={width}
          gap={gap}
          strokeWidth={strokeWidth}
          fill={fill}
        />
      ))}

      {Array(Math.floor(width/32)).fill(0).map((_, i) => (
        <VerticallLine
          key={i}
          nth={i}
          length={height}
          gap={gap}
          strokeWidth={strokeWidth}
          fill={fill}
        />
      ))}
    </React.Fragment>
  );
}

interface LineProps {
  nth: number,
  length: number,
  gap: number,
  strokeWidth: number,
  fill: string
}

function HorizontalLine({ nth, length, gap, strokeWidth, fill }: LineProps) {
  const x = 0;
  const y = gap * nth;
  const coordinate = `M${x} ${y}H${length}V${y+strokeWidth}H${x}V${y}Z`;
  return (
    <path d={coordinate} fill={fill}/>
  );
}

function VerticallLine({ nth, length, gap, strokeWidth, fill }: LineProps) {
  const x = gap * nth;
  const y = 0;
  const coordinate = `M${x} ${y}H${x+strokeWidth}V${y+length}H${x}V${y}Z`;
  return (
    <path d={coordinate} fill={fill}/>
  );
}