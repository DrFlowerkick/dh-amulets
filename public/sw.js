// Versioned cache name (generated via build.rs from Cargo.toml)
const CACHE_VERSION = "0.2.2"; // x-release-please-version
const CACHE_NAME = `drachenhueter-amulet-setup-v${CACHE_VERSION}`;

// Static assets to pre-cache
const ASSETS = [
  "/",
  "/pkg/dh-amulets.wasm",
  "/pkg/dh-amulets.js",
  "/pkg/dh-amulets.css",
  // Amulet images
  "/images/amulets/amulet_01.png",
  "/images/amulets/amulet_04.png",
  "/images/amulets/amulet_06.png",
  "/images/amulets/amulet_08.png",
  "/images/amulets/amulet_10.png",
  "/images/amulets/amulet_12.png",
  "/images/amulets/amulet_16.png",
  "/images/amulets/amulet_20.png",
  // Icons and screenshots
  "/favicon.ico",
  "/icon-192.png",
  "/icon-512.png",
  "/Screenshot_DH_20250713_Home.png",
  "/Screenshot_DH_20250713_Three_Player.png",
  "/Screenshot_DH_20250713_Home_Mobil.png",
  "/Screenshot_DH_20250713_Three_Player_Mobil.png",
];

// INSTALL: Cache all defined assets
self.addEventListener("install", (event) => {
  console.log("[SW] Installing service worker...");
  self.skipWaiting(); // activate immediately
  event.waitUntil(
    caches
      .open(CACHE_NAME)
    caches
      .open(CACHE_NAME)
      .then((cache) => cache.addAll(ASSETS))
      .catch((err) => console.error("[SW] Cache error during install:", err))
  );
});

// ACTIVATE: Delete outdated caches
self.addEventListener("activate", (event) => {
  console.log("[SW] Activating service worker...");
  event.waitUntil(
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key !== CACHE_NAME)
            .map((key) => {
              console.log("[SW] Deleting old cache:", key);
              return caches.delete(key);
            })
        )
      )
      .then(() => self.clients.claim())
    caches
      .keys()
      .then((keys) =>
        Promise.all(
          keys
            .filter((key) => key !== CACHE_NAME)
            .map((key) => {
              console.log("[SW] Deleting old cache:", key);
              return caches.delete(key);
            })
        )
      )
      .then(() => self.clients.claim())
  );
});

// FETCH: Serve from cache first, then fallback to network
self.addEventListener("fetch", (event) => {
  // Ignore non-GET requests
  if (event.request.method !== "GET") return;

  event.respondWith(
    caches.match(event.request).then((cached) => {
      if (cached) {
        return cached;
      }

      return fetch(event.request).catch((err) => {
        console.warn("[SW] Network fetch failed:", err);
        // Optionally return offline fallback here
      });
    })
  );
});
