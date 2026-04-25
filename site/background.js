(() => {
  const streams = Array.from(document.querySelectorAll(".binary-stream"));
  const liveBlockEl = document.querySelector("[data-live-block]");
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  const setupCursorEffects = () => {
    const canHover = window.matchMedia("(hover: hover) and (pointer: fine)").matches;

    if (reducedMotion || !canHover) {
      return;
    }

    const cursorLayer = document.createElement("div");
    cursorLayer.className = "cursor-effects";
    cursorLayer.setAttribute("aria-hidden", "true");
    cursorLayer.innerHTML = '<span class="cursor-ring"></span><span class="cursor-orb"></span><span class="cursor-trail trail-one"></span><span class="cursor-trail trail-two"></span>';
    document.body.append(cursorLayer);
    document.body.classList.add("has-cursor-effects");

    const parts = Array.from(cursorLayer.children);
    const state = {
      x: window.innerWidth / 2,
      y: window.innerHeight / 2,
      visible: false,
    };

    let frame = 0;

    const draw = () => {
      frame = 0;

      parts.forEach((part, index) => {
        const delay = index * 0.06;
        part.style.setProperty("--cursor-x", `${state.x}px`);
        part.style.setProperty("--cursor-y", `${state.y}px`);
        part.style.setProperty("--cursor-delay", `${delay}s`);
      });
    };

    const requestDraw = () => {
      if (!frame) {
        frame = window.requestAnimationFrame(draw);
      }
    };

    window.addEventListener("pointermove", (event) => {
      state.x = event.clientX;
      state.y = event.clientY;

      if (!state.visible) {
        cursorLayer.classList.add("is-visible");
        state.visible = true;
      }

      requestDraw();
    });

    window.addEventListener("pointerleave", () => {
      cursorLayer.classList.remove("is-visible", "is-active");
      state.visible = false;
    });

    const activeTargets = document.querySelectorAll("a, button, article, .architecture-panel, .architecture-blueprint, .arch-topology, .arch-layers, .arch-metrics div, .arch-layer, .layer, .invertx-focus, .metric-row div, .manual-grid div, .split, .faucet-form");
    activeTargets.forEach((target) => {
      target.addEventListener("pointerenter", () => cursorLayer.classList.add("is-active"));
      target.addEventListener("pointerleave", () => cursorLayer.classList.remove("is-active"));
    });
  };

  setupCursorEffects();

  if (!streams.length && !liveBlockEl) {
    return;
  }

  let block = 98402;
  let tx = 241;

  const toBinary = (value) => value.toString(2).padStart(8, "0").slice(-8);
  const shortHash = (value) => `0x${value.toString(16).padStart(5, "0")}`;

  const renderers = [
    (offset) => [
      toBinary(block + offset),
      toBinary(tx + offset * 3),
      toBinary(block + tx + offset * 5),
    ].join("  "),
    (offset) => [
      toBinary(tx + offset),
      toBinary(block - offset),
      toBinary(tx * 2 + offset * 7),
    ].join("  "),
    (offset) => `block ${block + offset}  tx ${shortHash(tx + offset)}  sync ${toBinary(tx + offset)}  finality ${toBinary(block + offset)}`,
  ];

  const render = () => {
    streams.forEach((stream, index) => {
      const offset = index * 13;
      stream.textContent = renderers[index % renderers.length](offset);
      stream.dataset.count = `+${String(tx + offset).padStart(6, "0")} tx`;
    });

    if (liveBlockEl) {
      liveBlockEl.textContent = block.toLocaleString("en-US");
    }

    block += 1;
    tx += 3;
  };

  render();

  if (!reducedMotion) {
    window.setInterval(render, 1400);
  }
})();
