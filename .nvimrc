vim.opt.wildignore:append({ "*/target/*", "*/node_modules/*", "*/pg_data/*" })

-- Rust-analyzer specific settings
vim.g.rust_analyzer = {
  ["rust-analyzer"] = {
	files = {
	  excludeDirs = { "target", "node_modules", "frontend/node_modules" }
	}
  }
}
