import { useState, useCallback } from "react";
import {
  ReactFlow,
  Controls,
  Background,
  applyNodeChanges,
  applyEdgeChanges,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";

const initialNodes = [
  { id: "1", position: { x: 0, y: 0 }, data: { label: "Upload file" } },
  { id: "2", position: { x: 0, y: 100 }, data: { label: "Detect mime type" } },
  { id: "3", position: { x: 0, y: 200 }, data: { label: "Validate QR data" } },
  {
    id: "A",
    // type: "group",
    data: { label: "For each page" },
    position: { x: 200, y: 50 },
    style: {
      width: 170,
      height: 140,
    },
  },
  {
    id: "B",
    data: { label: "OCR pages" },
    position: { x: 10, y: 35 },
    parentId: "A",
    extent: "parent",
  },
  {
    id: "C",
    data: { label: "Enrich e-mail data" },
    position: { x: 10, y: 90 },
    parentId: "A",
    extent: "parent",
  },
  { id: "4", position: { x: 400, y: 50 }, data: { label: "Persist data" } },
];
const initialEdges = [
  { id: "e1-2", source: "1", target: "2", type: "step" },
  { id: "e2-3", source: "2", target: "3", type: "step" },
  {
    id: "e3-A",
    source: "3",
    target: "A",
    type: "step",
  },
  { id: "A-e4", source: "A", target: "4", type: "step" },
];

export default function App() {
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);

  const onNodesChange = useCallback(
    (changes) => setNodes((nds) => applyNodeChanges(changes, nds)),
    [],
  );
  const onEdgesChange = useCallback(
    (changes) => setEdges((eds) => applyEdgeChanges(changes, eds)),
    [],
  );

  return (
    <div style={{ width: "100vw", height: "100vh", color: "black !important" }}>
      <ReactFlow
        colorMode="system"
        nodes={nodes}
        onNodesChange={onNodesChange}
        edges={edges}
        onEdgesChange={onEdgesChange}
        fitView
        defaultEdgeOptions={{ animated: true }}
      >
        <Background />
        <Controls />
      </ReactFlow>
    </div>
  );
}
