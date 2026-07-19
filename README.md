Organic LSP
====================

## What is it?

A language server for [the Organic programming language](https://github.com/ERSUCC/Organic).  Mainly tested in Neovim.

## How do I set it up?

### Neovim

Build the project with `cargo build --release`.  Then, add the following to your Neovim config (with the path filled in):

```lua
vim.filetype.add({
  extension = {
    organic = "organic",
  },
})

vim.lsp.config("organic-lsp", {
  cmd = { "/PATH/TO/organic-lsp/target/release/organic-lsp" },
  filetypes = { "organic" },
  root_markers = { ".git" },
})

vim.lsp.enable("organic-lsp")
```
