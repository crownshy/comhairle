const TARGET_DOMAIN = 'https://cdn.example.com';

self.addEventListener('install', event => {
  // Activate immediately
  self.skipWaiting();
});

self.addEventListener('activate', event => {
  // Take control of all pages
  event.waitUntil(self.clients.claim());
});

self.addEventListener('fetch', event => {
  const { request } = event;

  const url = new URL(request.url);
  console.log("URL ", url)

  // Only rewrite requests that are:
  // - Same-origin (excluding external scripts/APIs)
  // - Or relative paths (e.g., /assets/img.png)
  if (self.location.pathname ==="/api/tools/polis/proxy/"){
    if (url.origin === self.origin || url.origin === location.origin) {
        const newUrl = `/api/tools/polis/proxy/${url.pathname}${url.search}${url.hash}`;
  
        const newRequest = new Request(newUrl, {
          method: request.method,
          headers: request.headers,
          mode: request.mode,
          credentials: request.credentials,
          redirect: request.redirect,
          referrer: request.referrer,
          body: request.method !== 'GET' && request.method !== 'HEAD' ? request.body : undefined,
        });

        event.respondWith(fetch(newRequest));
    }
    
  }

});

