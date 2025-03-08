import init, { render } from "./app.js"

const CACHE_NAME = "pp_cache"

self.addEventListener("install", event => {
  console.log("installing")
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => {
      return cache.addAll([
        "favicon.ico",
        "manifest.json",
        "worker.js",
        "app.js",
        "app_bg.wasm",
        "https://unpkg.com/htmx.org@2.0.4/dist/htmx.min.js",
      ])
    })
  )
})

self.addEventListener("activate", event => {
  console.log("activating...")
  event.waitUntil(init())
})

self.addEventListener("fetch", event => {
  event.respondWith(intercept(event))
})

const REQUEST_PRIORITY = [fromCache, fromWasm, fromNetwork, fromUnable]

/**
 * @param {Event} event
 * @returns {Promise<Response>}
 */
async function intercept(event) {
  for (const method of REQUEST_PRIORITY) {
    const response = await method(event.request)

    console.log("trying", method.name, "for", event.request.url)

    if (response?.ok) {
      console.log(method.name, "was ok for", event.request.url)
      return response
    }
  }
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
 * @returns {Promise<Response>}
 */
async function fromUnable(req) {
  return new Response(`Network error happened ${req.method} - ${req.url}`, {
    headers: { "Content-Type": "text/plain" },
  })
}

/**
 * @param {Request} req
 * @returns {Promise<Response>}
 */
async function fromWasm(req) {
  const form = await getFormData(req)

  const response = await render(req.method, stripUrlHost(req), form)

  return response
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
  if (req.method !== "POST") {
    return null
  }

  const form = await req.formData()

  return new URLSearchParams([...form]).toString()
}
