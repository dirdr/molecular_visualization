import init, { MoleculeViewer } from "./pkg/molecular_visualization.js";

let initialized;

export async function mountMolecule(canvas, pdb, onError = () => {}) {
  initialized ??= init().catch((error) => {
    initialized = undefined;
    throw error;
  });
  await initialized;
  let viewer = new MoleculeViewer(canvas, pdb);
  const controller = new AbortController();
  const options = { signal: controller.signal };
  let frame = 0;
  let pointer;
  let lost = false;
  const schedule = () => {
    if (frame || lost || !viewer) return;
    frame = requestAnimationFrame(() => {
      frame = 0;
      if (!viewer || lost) return;
      const ratio = Math.min(devicePixelRatio || 1, 2);
      try {
        viewer.draw(
          Math.max(1, Math.round(canvas.clientWidth * ratio)),
          Math.max(1, Math.round(canvas.clientHeight * ratio)),
        );
      } catch (error) {
        onError(error);
      }
    });
  };
  const position = (event) => {
    const rect = canvas.getBoundingClientRect();
    return [event.clientX - rect.left, event.clientY - rect.top];
  };
  canvas.addEventListener(
    "pointerdown",
    (event) => {
      if (event.button !== 0 || pointer !== undefined || lost || !viewer)
        return;
      event.preventDefault();
      canvas.focus({ preventScroll: true });
      pointer = event.pointerId;
      canvas.setPointerCapture(pointer);
      viewer.pointer_down(...position(event));
    },
    options,
  );
  canvas.addEventListener(
    "pointermove",
    (event) => {
      if (pointer !== event.pointerId || lost || !viewer) return;
      viewer.pointer_move(...position(event));
      schedule();
    },
    options,
  );
  const release = (event) => {
    if (pointer !== event.pointerId) return;
    pointer = undefined;
    viewer?.pointer_up();
    if (canvas.hasPointerCapture(event.pointerId))
      canvas.releasePointerCapture(event.pointerId);
  };
  for (const name of ["pointerup", "pointercancel", "lostpointercapture"])
    canvas.addEventListener(name, release, options);
  canvas.addEventListener(
    "wheel",
    (event) => {
      if (lost || !viewer) return;
      event.preventDefault();
      viewer.zoom(event.deltaY);
      schedule();
    },
    { ...options, passive: false },
  );
  canvas.addEventListener(
    "keydown",
    (event) => {
      if (lost || !viewer) return;
      const direction = {
        ArrowLeft: [-20, 0],
        ArrowRight: [20, 0],
        ArrowUp: [0, -20],
        ArrowDown: [0, 20],
      }[event.key];
      if (direction) {
        const x = canvas.clientWidth / 2,
          y = canvas.clientHeight / 2;
        viewer.pointer_down(x, y);
        viewer.pointer_move(x + direction[0], y + direction[1]);
        viewer.pointer_up();
      } else if (event.key === "+" || event.key === "=") viewer.zoom(-100);
      else if (event.key === "-") viewer.zoom(100);
      else if (event.key.toLowerCase() === "r") viewer.reset_view();
      else return;
      event.preventDefault();
      schedule();
    },
    options,
  );
  canvas.addEventListener(
    "webglcontextlost",
    (event) => {
      event.preventDefault();
      lost = true;
      pointer = undefined;
      onError(new Error("Graphics context lost. Waiting to restore it."));
    },
    options,
  );
  canvas.addEventListener(
    "webglcontextrestored",
    () => {
      viewer?.free();
      viewer = undefined;
      try {
        viewer = new MoleculeViewer(canvas, pdb);
        lost = false;
        onError(null);
        schedule();
      } catch (error) {
        onError(error);
      }
    },
    options,
  );
  const observer = new ResizeObserver(schedule);
  observer.observe(canvas);
  schedule();
  return () => {
    controller.abort();
    observer.disconnect();
    cancelAnimationFrame(frame);
    if (pointer !== undefined && canvas.hasPointerCapture(pointer))
      canvas.releasePointerCapture(pointer);
    viewer?.free();
    viewer = undefined;
  };
}
