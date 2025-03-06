import init, { render } from './app.js'

const CACHE_NAME = 'pp_cache'

self.addEventListener('install', event => {
  console.log("installing")
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => {
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

self.addEventListener('activate', event => {
  console.log("activating...")
  event.waitUntil(init())
})

self.addEventListener('fetch', event => {
  const path = event.request.url
  const origin = `${self.location.origin}/fragments`

  if (path.startsWith(origin)) {
    const response = requestFromWasm(event.request)
    event.respondWith(response)
  } else {
    const response = requestFromNetwork(event)
    event.respondWith(response)
  }
})

/**
 * @param {Event} event
 * @returns {Promise<Response>}
 */
 async function requestFromNetwork(event) {
  try {
    const response = await fetch(event.request)
    event.waitUntil(putInCache(event.request, response.clone()))
    return response
  } catch (error) {
    console.error(error)
    const response = await caches.match(event.request)

    if (response) {
      return response
    }
    return new Response("Network error happened", {
      status: 408,
      headers: { "Content-Type": "text/plain" },
    })
  }
}

async function putInCache(request, response) {
  const cache = await caches.open(CACHE_NAME)
  await cache.put(request, response)
}

/**
 * @param {Request} req
 * @returns {Promise<Response>}
 */
 async function requestFromWasm(req) {
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
 * @returns {Promise<string?>}
 */
 async function getFormData(req) {
  if (req.method !== "POST") {
    return null
  }

  const form = await req.formData()

  return new URLSearchParams([...form]).toString()
}
