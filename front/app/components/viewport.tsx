"use client";

import {
  useId,
  useRef,
  type MouseEvent,
  type PropsWithChildren,
  type WheelEvent,
} from "react";

interface Point {
  x: number;
  y: number;
}

interface Camera extends Point {
  zoom: number;
}

export function Viewport({ children }: PropsWithChildren) {
  const id = useId();
  const viewportRef = useRef<HTMLDivElement>(null);
  const cameraRef = useRef<Camera>({ x: 0, y: 0, zoom: 1 });
  const panningRef = useRef<{ start: Point; active: boolean }>({
    start: { x: 0, y: 0 },
    active: false,
  });

  const cssVars = {
    cameraX: `--camera-${id}-x`,
    cameraY: `--camera-${id}-y`,
    cameraZoom: `--camera-${id}-zoom`,
    cursor: `--cursor-${id}`,
  };

  const setCss = (name: string, value: string) => {
    viewportRef.current?.style.setProperty(name, value);
  };

  const setCamera = (camera: Camera) => {
    cameraRef.current = camera;
    setCss(cssVars.cameraX, `${camera.x}px`);
    setCss(cssVars.cameraY, `${camera.y}px`);
    setCss(cssVars.cameraZoom, `${camera.zoom}`);
  };

  const handleWheel = (e: WheelEvent) => {
    const camera = cameraRef.current;

    const zoomSpeed = 0.001;
    const delta = -e.deltaY * zoomSpeed;

    const zoom = Math.max(0.5, Math.min(3, camera.zoom + delta));
    const x = e.clientX - (e.clientX - camera.x) * (zoom / camera.zoom);
    const y = e.clientY - (e.clientY - camera.y) * (zoom / camera.zoom);

    setCamera({ x, y, zoom });
  };

  const handlePointerDown = (e: MouseEvent) => {
    if (e.button === 0) {
      const camera = cameraRef.current;

      panningRef.current.start = {
        x: e.clientX - camera.x,
        y: e.clientY - camera.y,
      };

      panningRef.current.active = true;
      setCss(cssVars.cursor, "grabbing");
    }
  };

  const handlePointerMove = (e: MouseEvent) => {
    if (panningRef.current.active) {
      const camera = cameraRef.current;
      const panStart = panningRef.current.start;

      setCamera({
        ...camera,
        x: e.clientX - panStart.x,
        y: e.clientY - panStart.y,
      });
    }
  };

  const handlePointerUp = () => {
    panningRef.current.active = false;
    setCss(cssVars.cursor, "grab");
  };

  return (
    <div
      className="flex-1 relative overflow-hidden touch-none"
      ref={viewportRef}
      onWheel={handleWheel}
      onPointerUp={handlePointerUp}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerLeave={handlePointerUp}
      style={{ cursor: `var(${cssVars.cursor})` }}
    >
      <div
        style={{
          pointerEvents: "none",
          transform: `translate(var(${cssVars.cameraX}), var(${cssVars.cameraY})) scale(var(${cssVars.cameraZoom}))`,
          transformOrigin: "0 0",
          width: "100%",
          height: "100%",
        }}
      >
        {children}
      </div>
    </div>
  );
}
