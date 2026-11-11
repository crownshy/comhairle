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
