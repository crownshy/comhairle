// The Handbook is static, public content (see +layout@.svelte, which already
// breaks out of the auth-gated (admin) layout into the (public) one). Prerender
// the whole subtree at build time.
export const prerender = true;
