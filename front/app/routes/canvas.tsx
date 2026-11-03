"use client";

import Canvas from "~/components/canvas.client";

export default function _Canvas() {
  if (!Canvas) return null;
  return <Canvas />;
}
