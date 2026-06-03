import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/app/**/*.{ts,tsx}",
    "./src/components/**/*.{ts,tsx}",
    "./src/hooks/**/*.{ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // brand palette — calm slate with an electric accent for the "instant" moments
        ink: "#0b0e14",
        panel: "#11151f",
        edge: "#1e2430",
        accent: "#5b8cff",
        good: "#3ddc97",
        warn: "#ffb454",
        bad: "#ff6b6b",
      },
      fontFamily: {
        mono: ["ui-monospace", "SFMono-Regular", "Menlo", "monospace"],
      },
    },
  },
  plugins: [],
} satisfies Config;
