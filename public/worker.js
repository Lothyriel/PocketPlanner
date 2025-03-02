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

self.addEventListener('fetch', event => {
  const path = event.request.url;
  const origin = `${self.location.origin}/fragments`

  if (path.startsWith(origin)) {
    const render = renderFromWasm(event.request);
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
 * @param {Request} req
 * @returns {Promise<Response>}
 */
async function renderFromWasm(req) {
  const form = await getFormData(req)
  const parts = { form, route: getRoute(req), method: req.method }
  const fragment = await render(parts)

  return new Response(fragment, {
    headers: { "Content-Type": "text/html" },
  })
}

/**
 * @param {Request} req
 * @returns {string}
 */
function getRoute(req) {
  const url = new URL(req.url)

  return url.pathname + url.search
}

/**
 * @param {Request} req
 * @returns {Promise<Object<string, string>>}
 */
async function getFormData(req) {
  if (req.method !== "POST") {
    return {}
  }

  const form = await req.formData();

  return Array.from(form).reduce((acc, [key, value]) => {
    acc[key] = value
    return acc
  }, {})
}
