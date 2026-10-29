// Shared by the workspace and conversation sidebars so their rows look the same.
// Never draw sidebar text on --sidebar-accent alone: in some themes (scot-gov) it is white.
export const ROW_HOVER = 'hover:bg-sidebar-foreground/10';
export const MENU_BUTTON_HOVER =
	'hover:bg-sidebar-foreground/10 hover:text-sidebar-foreground active:bg-sidebar-foreground/10 active:text-sidebar-foreground data-[state=open]:bg-sidebar-foreground/10 data-[state=open]:hover:bg-sidebar-foreground/10 data-[state=open]:hover:text-sidebar-foreground';
export const ROW_ACTIVE = 'bg-sidebar-accent text-sidebar-accent-foreground font-medium';
export const LINK_ROW = 'flex h-9 items-center gap-2 rounded-lg px-2 text-sm';
export const LINK_ROW_IDLE = 'text-sidebar-foreground/80 hover:text-sidebar-foreground';
