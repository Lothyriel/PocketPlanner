self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open('pp_cache').then((cache) => {
      return cache.addAll([
        '/index.html',
        '/wasm.js',
        '/worker.js',
        '/app.wasm',
      ])
    })
  )
})

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request)
      .then((cachedResponse) => {
        return cachedResponse || fetch(event.request)
      })
  )
})
