/* INFI theme handler
 * - Reads stored preference, falls back to prefers-color-scheme on first visit
 * - Updates <html data-theme=...>, the meta theme-color, and aria-pressed on the toggle
 * - The boot logic in index.html runs before paint to prevent a flash of wrong theme
 */
(() => {
  const STORAGE_KEY = "infi-theme";
  const root = document.documentElement;

  const apply = (theme) => {
    root.setAttribute("data-theme", theme);
    document.querySelectorAll('meta[name="theme-color"]').forEach((m) => {
      m.setAttribute("content", theme === "light" ? "#f3f7f5" : "#02070c");
    });
    document.querySelectorAll('[data-theme-toggle]').forEach((btn) => {
      btn.setAttribute("aria-pressed", theme === "light" ? "true" : "false");
      btn.setAttribute(
        "aria-label",
        theme === "light" ? "Switch to dark mode" : "Switch to light mode"
      );
      btn.setAttribute(
        "title",
        theme === "light" ? "Switch to dark mode" : "Switch to light mode"
      );
    });
  };

  const stored = (() => {
    try {
      return localStorage.getItem(STORAGE_KEY);
    } catch (e) {
      return null;
    }
  })();

  const initial =
    stored === "light" || stored === "dark"
      ? stored
      : window.matchMedia("(prefers-color-scheme: light)").matches
        ? "light"
        : "dark";

  apply(initial);

  // Listen for system preference changes — only apply if user hasn't explicitly chosen
  if (window.matchMedia) {
    const mql = window.matchMedia("(prefers-color-scheme: light)");
    const onChange = (e) => {
      try {
        if (!localStorage.getItem(STORAGE_KEY)) {
          apply(e.matches ? "light" : "dark");
        }
      } catch (_) {}
    };
    if (mql.addEventListener) mql.addEventListener("change", onChange);
    else if (mql.addListener) mql.addListener(onChange);
  }

  // Wire up the toggle buttons
  const init = () => {
    document.querySelectorAll("[data-theme-toggle]").forEach((btn) => {
      btn.addEventListener("click", () => {
        const next =
          root.getAttribute("data-theme") === "light" ? "dark" : "light";
        apply(next);
        try {
          localStorage.setItem(STORAGE_KEY, next);
        } catch (_) {}
      });
    });
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", init, { once: true });
  } else {
    init();
  }
})();
