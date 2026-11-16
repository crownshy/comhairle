// The Comhairle colours, read from the theme file itself so they never drift from it.
//
// Why: the app's own theme comes from PUBLIC_THEME (Scottish Government locally) and the
// server puts it on <html data-theme>. Every other theme is an attribute selector, so the
// prototype can set it on its wrapper in the server render. Comhairle is the bare :root, which
// can't be re-applied that way, so the page arrived blue and only turned green once the
// browser took over. Putting these declarations on the wrapper as an inline style makes the
// very first paint green.
import themeCss from '$lib/styles/comhairle-themes.css?raw';

const block = themeCss.match(/\/\* Base Theme[^*]*\*\/\s*:root\s*\{([^}]*)\}/)?.[1] ?? '';

/** The light-mode Comhairle custom properties as one inline style string. */
export const COMHAIRLE_VARS = block
	.replace(/\/\*[\s\S]*?\*\//g, '')
	.split(';')
	.map((declaration) => declaration.trim())
	.filter((declaration) => declaration.startsWith('--'))
	.join('; ');
