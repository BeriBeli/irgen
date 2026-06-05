(() => {
  const root = document.documentElement;
  const themeToggle = document.getElementById("theme-toggle");
  const searchInput = document.getElementById("register-search");
  const searchResults = document.getElementById("search-results");
  let highlightedRegister = null;
  let searchIndex = null;

  function readJson(id, fallback) {
    const element = document.getElementById(id);
    if (!element) {
      return fallback;
    }
    try {
      return JSON.parse(element.textContent || "");
    } catch {
      return fallback;
    }
  }

  function getSearchIndex() {
    if (!searchIndex) {
      searchIndex = readJson("register-search-index", []).map((entry) => ({
        ...entry,
        lowerSearch: String(entry.search || "").toLowerCase(),
      }));
    }
    return searchIndex;
  }

  function savedTheme() {
    try {
      return window.localStorage.getItem("irgen-docs-theme");
    } catch {
      return null;
    }
  }

  function saveTheme(theme) {
    try {
      window.localStorage.setItem("irgen-docs-theme", theme);
    } catch {
      return;
    }
  }

  function preferredTheme() {
    const saved = savedTheme();
    if (saved === "dark" || saved === "light") {
      return saved;
    }
    return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }

  function applyTheme(theme) {
    root.dataset.theme = theme;
    if (themeToggle) {
      themeToggle.checked = theme === "dark";
    }
  }

  function clearHighlight() {
    if (highlightedRegister) {
      highlightedRegister.classList.remove("register-section-active");
      highlightedRegister = null;
    }
  }

  function highlightFromHash() {
    clearHighlight();
    const id = window.location.hash.slice(1);
    if (!id) {
      return;
    }
    const element = document.getElementById(id);
    const register = element ? element.closest("[data-register]") : null;
    if (register) {
      highlightedRegister = register;
      highlightedRegister.classList.add("register-section-active");
      window.requestAnimationFrame(() => {
        element.scrollIntoView({ block: "start" });
      });
    }
  }

  function clearSearchResults() {
    if (searchResults) {
      searchResults.replaceChildren();
    }
  }

  function appendSearchResult(register) {
    const link = document.createElement("a");
    const name = document.createElement("span");
    const offset = document.createElement("span");

    link.className = "search-result";
    link.setAttribute("role", "option");
    link.href = register.href || `#${register.anchor}`;
    name.className = "search-result-name";
    offset.className = "search-result-offset";
    name.textContent = register.name || register.anchor;
    offset.textContent = register.offset || "";

    link.append(name, offset);
    searchResults.append(link);
  }

  function updateSearch() {
    clearSearchResults();
    const query = searchInput ? searchInput.value.trim().toLowerCase() : "";
    if (!query || !searchResults) {
      return;
    }

    const matches = getSearchIndex()
      .filter((register) => register.lowerSearch.includes(query))
      .slice(0, 24);

    if (matches.length === 0) {
      const empty = document.createElement("div");
      empty.className = "search-empty";
      empty.textContent = "No matches";
      searchResults.append(empty);
      return;
    }

    matches.forEach(appendSearchResult);
  }

  if (themeToggle) {
    applyTheme(preferredTheme());
    themeToggle.addEventListener("change", () => {
      const theme = themeToggle.checked ? "dark" : "light";
      saveTheme(theme);
      applyTheme(theme);
    });
  } else {
    applyTheme(preferredTheme());
  }

  if (searchInput) {
    searchInput.addEventListener("input", updateSearch);
    searchInput.addEventListener("keydown", (event) => {
      if (event.key === "Escape") {
        searchInput.value = "";
        clearSearchResults();
        clearHighlight();
      }
    });
  }

  window.addEventListener("hashchange", highlightFromHash);
  highlightFromHash();
})();
