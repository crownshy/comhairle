/** Report rich text reads at text-lg, a step up from the editor's default. */
export const REPORT_TEXT_CLASS =
	'text-foreground text-lg leading-7 [&_.tiptap]:text-lg [&_.tiptap]:leading-7 [&_.tiptap_h3]:text-xl [&_.tiptap_h3]:font-semibold [&_.tiptap_li]:my-2 [&_.tiptap_p]:my-3 [&_.tiptap_p]:text-lg [&_.tiptap_p]:leading-7';

/** The reading column every report screen sits in. */
export const REPORT_COLUMN_CLASS = 'mx-auto w-full max-w-2xl px-5';

/** The small uppercase label above a block, such as "Executive summary". */
export const REPORT_EYEBROW_CLASS = 'text-primary text-sm font-medium tracking-wide uppercase';

/** One section pill in the report header (an H2 in the body, ADR-0058). */
export const REPORT_PILL_CLASS =
	'shrink-0 rounded-full px-3 py-1.5 text-sm font-medium shadow-xs transition-colors';

/**
 * {@link REPORT_TEXT_CLASS} for the report editor. The editor keeps embedded charts inside its
 * document, so these rules only touch its own top-level blocks (`.tiptap > p`) and leave the
 * charts styled as readers see them.
 */
export const REPORT_EDITOR_TEXT_CLASS =
	'text-foreground text-lg leading-7 [&_.tiptap]:text-lg [&_.tiptap]:leading-7 [&_.tiptap>h3]:text-xl [&_.tiptap>h3]:font-semibold [&_.tiptap>:is(ul,ol)>li]:my-2 [&_.tiptap>p]:my-3 [&_.tiptap>p]:text-lg [&_.tiptap>p]:leading-7';

/** In the editor, an H2 starts a new section, so it is set like a card title with room above. */
export const REPORT_EDITOR_SECTION_CLASS =
	'[&_.tiptap>h2]:text-muted-foreground [&_.tiptap>h2]:mt-14 [&_.tiptap>h2]:border-t [&_.tiptap>h2]:pt-8 [&_.tiptap>h2]:text-2xl [&_.tiptap>h2]:leading-8 [&_.tiptap>h2]:font-semibold [&_.tiptap>h2:first-child]:mt-0 [&_.tiptap>h2:first-child]:border-t-0 [&_.tiptap>h2:first-child]:pt-0 [&_.tiptap>p_em]:text-muted-foreground';

/** Each editable block tints on hover, so it's clear the text can be changed. */
export const REPORT_EDITOR_HOVER_CLASS =
	'[&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:-mx-3 [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:cursor-text [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:rounded-lg [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:px-3 [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:transition-colors [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote):hover]:bg-accent/60';
