(() => {
  const streams = Array.from(document.querySelectorAll(".binary-stream"));
  const liveBlockEl = document.querySelector("[data-live-block]");
  const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

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
