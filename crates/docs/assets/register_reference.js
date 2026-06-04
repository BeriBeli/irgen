(() => {
  const root = document.documentElement;
  const pages = Array.from(document.querySelectorAll("[data-page]"));
  const pageLinks = Array.from(document.querySelectorAll(".toc-link[data-target]"));
  const registers = Array.from(document.querySelectorAll("[data-register]"));
  const themeToggle = document.getElementById("theme-toggle");
  const searchInput = document.getElementById("register-search");
  const searchResults = document.getElementById("search-results");
  let highlightedRegister = null;

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

  function showPage(id) {
    const target = pages.find((page) => page.dataset.page === id) || pages[0];
    if (!target) {
      return;
    }

    pages.forEach((page) => {
      page.classList.toggle("page-active", page === target);
    });
    pageLinks.forEach((link) => {
      link.classList.toggle(
        "toc-link-active",
        link.dataset.target === target.dataset.page
      );
    });

    return target;
  }

  function highlightRegister(element) {
    if (highlightedRegister && highlightedRegister !== element) {
      highlightedRegister.classList.remove("register-section-active");
    }
    highlightedRegister = element && element.matches("[data-register]") ? element : null;
    if (highlightedRegister) {
      highlightedRegister.classList.add("register-section-active");
    }
  }

  function showAnchor(id, updateHash) {
    const fallback = pages[0];
    const element = id ? document.getElementById(id) : fallback;
    const page = element ? element.closest("[data-page]") : fallback;
    const shownPage = showPage(page ? page.dataset.page : "summary");

    if (updateHash && id) {
      window.history.replaceState(null, "", `#${id}`);
    }

    window.requestAnimationFrame(() => {
      if (element && element !== shownPage) {
        element.scrollIntoView({ block: "start" });
      }
      highlightRegister(element ? element.closest("[data-register]") : null);
    });
  }

  function clearSearchResults() {
    if (searchResults) {
      searchResults.replaceChildren();
    }
  }

  function appendSearchResult(register) {
    const button = document.createElement("button");
    const name = document.createElement("span");
    const offset = document.createElement("span");

    button.type = "button";
    button.className = "search-result";
    button.setAttribute("role", "option");
    name.className = "search-result-name";
    offset.className = "search-result-offset";
    name.textContent = register.dataset.registerName || register.id;
    offset.textContent = register.dataset.registerOffset || "";

    button.append(name, offset);
    button.addEventListener("click", () => {
      showAnchor(register.id, true);
    });
    searchResults.append(button);
  }

  function updateSearch() {
    clearSearchResults();
    const query = searchInput ? searchInput.value.trim().toLowerCase() : "";
    if (!query || !searchResults) {
      return;
    }

    const matches = registers
      .filter((register) => (register.dataset.search || "").toLowerCase().includes(query))
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

  pageLinks.forEach((link) => {
    link.addEventListener("click", (event) => {
      event.preventDefault();
      showAnchor(link.dataset.target, true);
    });
  });

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
        highlightRegister(null);
      }
    });
  }

  window.addEventListener("hashchange", () => {
    showAnchor(window.location.hash.slice(1) || "summary", false);
  });

  showAnchor(window.location.hash.slice(1) || "summary", false);
})();
