import init, { render } from "./app.js"

const CACHE_NAME = "pp_cache"

const CACHED_ASSETS = [
  "icon-192.png",
  "icon-512.png",
  "favicon.ico",
  "manifest.json",
  "worker.js",
  "app.js",
  "app_bg.wasm",
  "https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js",
]

self.addEventListener("install", event => {
  console.log("installing")
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => {
      return cache.addAll(CACHED_ASSETS)
    })
  )
})

self.addEventListener("activate", event => {
  console.log("activating...")
  event.waitUntil(clearCache())
})

self.addEventListener("fetch", event => {
  event.respondWith(intercept(event))
})

async function clearCache() {
  return Promise.all(CACHED_ASSETS.map(n => caches.delete(n)))
}

const REQUEST_PRIORITY = [fromCache, fromWasm, fromNetwork]

/**
 * @param {Event} event
 * @returns {Promise<Response>}
 */
async function intercept(event) {
  for (const method of REQUEST_PRIORITY) {
    const response = await method(event.request)

    if (response?.ok) {
      return response
    }
  }

  return offlineResponse(req)
}

/**
 * @param {Request} req
 * @returns {Promise<Response?>}
 */
function fromCache(req) {
  return caches.match(req)
}

/**
 * @param {Request} req
 * @returns {Promise<Response>}
 */
function fromNetwork(req) {
  return fetch(req)
}

/**
 * @param {Request} req
 * @returns {Response}
 */
function offlineResponse(req) {
  return new Response(`Network error happened ${req.method} - ${req.url}`, {
    headers: { "Content-Type": "text/plain" },
  })
}

/**
 * @param {Request} req
 * @returns {Promise<Response>}
 */
async function fromWasm(req) {
  await init()

  const params = {
    method: req.method,
    url: stripUrlHost(req),
    form: await getFormData(req),
    headers: Object.fromEntries(req.headers),
  }

  return await render(params)
}

/**
 * @param {Request} req
 * @returns {string}
 */
function stripUrlHost(req) {
  const url = new URL(req.url)

  return url.pathname + url.search
}

/**
 * @param {Request} req
 * @returns {Promise<string?>}
 */
async function getFormData(req) {
  if (req.method === "GET") {
    return null
  }

  const form = await req.formData()

  return new URLSearchParams([...form]).toString()
}
