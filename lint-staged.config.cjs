module.exports = {
  "*.{js,jsx,ts,tsx}": ["eslint --fix", "prettier --write"],
  "*.{json,toml,yml,yaml}": "prettier --write",
  "*.rs": "rustfmt",
  "*": "cspell --no-must-find-files",
};
