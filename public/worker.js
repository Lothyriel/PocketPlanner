import init, { render } from './app.js'

self.addEventListener('install', (event) => {
  (async function () {
    await init()
  })()

  console.log("Registered worker with wasm module")

  event.waitUntil(
    caches.open('pp_cache').then((cache) => {
      return cache.addAll([
        'index.html',
        'worker.js',
        'app.js',
        'app_bg.wasm',
        'https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js',
      ])
    })
  )
})

self.addEventListener('fetch', (event) => {
  const path = event.request.url;
  const origin = `${self.location.origin}/fragments`

  if (path.startsWith(origin)) {
    console.log("Intercepting ", path.slice(origin.length))
    const render = renderFromWasm(path.slice(origin.length));
    console.log("Response, ", render)
    event.respondWith(render)
  }
})

/**
 * @param {string} path
 * @returns {Response}
 */
function renderFromWasm(path) {
  const fragment = render(path)

  console.log("Returning frag ", fragment)

  return new Response(fragment, {
    headers: { "Content-Type": "text/html" },
  })
}
