const CACHE_NAME = "leptos-app-v1";
const ASSETS = [
  "/",
  "/pkg/dh-amulets.wasm",
  "/pkg/dh-amulets.js",
  "/pkg/dh-amulets.css",
  "/images/amulets/amulet_01.png",
  "/images/amulets/amulet_04.png",
  "/images/amulets/amulet_06.png",
  "/images/amulets/amulet_08.png",
  "/images/amulets/amulet_10.png",
  "/images/amulets/amulet_12.png",
  "/images/amulets/amulet_16.png",
  "/images/amulets/amulet_20.png",
  "/favicon.ico",
  "/icon-192.png",
  "/icon-512.png",
  "/Screenshot_DH_20250713_Home.png",
  "/Screenshot_DH_20250713_Three_Player.png",
  "/Screenshot_DH_20250713_Home_Mobil.png",
  "/Screenshot_DH_20250713_Three_Player_Mobil.png",
  "/sw.js",
  "/manifest.json",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(ASSETS))
  );
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((cached) => {
      return cached || fetch(event.request);
    })
  );
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) =>
      Promise.all(
        keys
          .filter((key) => key !== CACHE_NAME)
          .map((key) => caches.delete(key))
      )
    )
  );
});
