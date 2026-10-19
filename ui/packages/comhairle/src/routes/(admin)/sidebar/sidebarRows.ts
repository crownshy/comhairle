// Shared by the workspace and conversation sidebars so their rows look the same.
// Never draw sidebar text on --sidebar-accent alone: in some themes (scot-gov) it is white.
export const ROW_HOVER = 'hover:bg-sidebar-foreground/10';
export const ROW_ACTIVE = 'bg-sidebar-accent text-sidebar-accent-foreground font-medium';
export const LINK_ROW = 'flex h-10 items-center gap-2 rounded-lg px-2 text-base';
export const LINK_ROW_IDLE = 'text-sidebar-foreground/80 hover:text-sidebar-foreground';
