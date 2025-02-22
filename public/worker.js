import init, { render } from './app.js'

const CACHE_NAME = 'pp_cache'

self.addEventListener('install', (event) => {
  (async function () {
    await init()
  })()

  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => {
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
    const render = renderFromWasm(path.slice(origin.length));
    event.respondWith(render)
  } else {
    fetch(event.request)
      .then((response) => {
        if (response.ok) {
          caches.open(CACHE_NAME).then((cache) => {
            cache.put(event.request, response.clone());
          });
        }
        return response;
      })
      .catch(() => {
        return caches.match(event.request);
      })
  }
})

/**
 * @param {string} path
 * @returns {Response}
 */
function renderFromWasm(path) {
  const fragment = render(path)

  return new Response(fragment, {
    headers: { "Content-Type": "text/html" },
  })
}
