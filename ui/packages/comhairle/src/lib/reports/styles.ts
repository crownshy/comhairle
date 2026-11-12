/** Report rich text reads at text-lg, a step up from the editor's default. */
export const REPORT_TEXT_CLASS =
	'text-foreground text-lg leading-7 [&_.tiptap]:text-lg [&_.tiptap]:leading-7 [&_.tiptap_h3]:text-xl [&_.tiptap_h3]:font-semibold [&_.tiptap_li]:my-2 [&_.tiptap_p]:my-3 [&_.tiptap_p]:text-lg [&_.tiptap_p]:leading-7';

/** The column every report screen sits in: wide enough for three stat cards in a row. */
export const REPORT_COLUMN_CLASS = 'mx-auto w-full max-w-4xl px-5';

/** The small uppercase label above a block, such as "Executive summary". */
export const REPORT_EYEBROW_CLASS = 'text-primary text-sm font-medium tracking-wide uppercase';

/** One page's pill in the report header (ADR-0069). */
export const REPORT_PILL_CLASS =
	'max-w-64 shrink-0 rounded-full px-3 py-1.5 text-sm font-medium shadow-xs transition-colors';

/**
 * {@link REPORT_TEXT_CLASS} for the report editor. The editor keeps embedded charts inside its
 * document, so these rules only touch its own top-level blocks (`.tiptap > p`) and leave the
 * charts styled as readers see them.
 */
export const REPORT_EDITOR_TEXT_CLASS =
	'text-foreground text-lg leading-7 [&_.tiptap]:text-lg [&_.tiptap]:leading-7 [&_.tiptap>h3]:text-xl [&_.tiptap>h3]:font-semibold [&_.tiptap>:is(ul,ol)>li]:my-2 [&_.tiptap>p]:my-3 [&_.tiptap>p]:text-lg [&_.tiptap>p]:leading-7';

/** In the editor, a page break (a horizontal rule) starts a new page, so it shows as a labelled line. */
export const REPORT_EDITOR_PAGE_BREAK_CLASS =
	"[&_.tiptap>hr]:relative [&_.tiptap>hr]:my-10 [&_.tiptap>hr]:overflow-visible [&_.tiptap>hr]:border-0 [&_.tiptap>hr]:border-t-2 [&_.tiptap>hr]:border-dashed [&_.tiptap>hr]:border-border [&_.tiptap>hr::after]:absolute [&_.tiptap>hr::after]:start-1/2 [&_.tiptap>hr::after]:-top-3 [&_.tiptap>hr::after]:-translate-x-1/2 [&_.tiptap>hr::after]:bg-background [&_.tiptap>hr::after]:px-3 [&_.tiptap>hr::after]:text-base [&_.tiptap>hr::after]:text-muted-foreground [&_.tiptap>hr::after]:content-['Page_break'] [&_.tiptap>hr.ProseMirror-selectednode]:border-primary";

/**
 * Each written block and chart tints on hover, so it's clear it can be changed. The negative margin
 * keeps the text lined up with the headings around it, and is important because the editor's
 * own unlayered `.tiptap p` margin otherwise wins over it.
 */
export const REPORT_EDITOR_HOVER_CLASS =
	'[&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:-mx-3! [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:cursor-text [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:rounded-lg [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:px-3 [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote)]:transition-colors [&_.tiptap>:is(p,h2,h3,ul,ol,blockquote):not(.is-editor-empty):hover]:bg-accent/60 [&_.tiptap>.report-embed--editor:hover]:bg-accent/60';
