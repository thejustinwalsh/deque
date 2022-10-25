/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
        accent: "var(--color-accent)",
        foreground: "var(--color-foreground)",
        background: "var(--color-background)",
        "background-100": "var(--color-background-100)",
        "background-200": "var(--color-background-200)",
        "background-300": "var(--color-background-300)",
        "background-contrast": "var(--color-background-contrast)",
      },
    },
    fontFamily: {
      sans: [
        "-apple-system",
        "BlinkMacSystemFont",
        "Segoe UI Variable Text",
        "system-ui",
        "sans-serif",
      ],
      mono: [
        "ui-monospace",
        "SF Mono",
        "SFMono-Regular",
        "Menlo",
        "Monaco",
        "Cascadia Mono",
        "Segoe UI Mono",
        "Roboto Mono",
        "Oxygen Mono",
        "Ubuntu Monospace",
        "Source Code Pro",
        "Fira Mono",
        "Droid Sans Mono",
        "Courier New",
        "monospace",
      ],
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
