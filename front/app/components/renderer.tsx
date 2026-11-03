"use client";

import { useEffect, useRef, type ComponentProps } from "react";
import { launch } from "vocab_map_renderer";

export function Renderer(props: ComponentProps<"div">) {
  const started = useRef(false);

  useEffect(() => {
    if (started.current) return;
    launch();
    started.current = true;
  }, []);

  return (
    <div {...props}>
      <template id="iced" />
    </div>
  );
}
