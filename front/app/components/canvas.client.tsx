"use client";

import { useEffect, useRef } from "react";
import { start_canvas } from "vocab_map_renderer";

export default function Canvas() {
  const started = useRef(false);
  useEffect(() => {
    if (started.current) return;
    start_canvas();
    started.current = true;
  }, []);

  return null;
}
