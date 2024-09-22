/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    colors: {
      secondary: "#1B1B1B",
      secondaryDark: "rgba(20,20,20,0.5)",
      accent: "#D5CDEE",
      primary: "#fff",
      primaryDark: "rgba(255,255,255,0.5)",
      border: "rgba(255,255,255,0.1)",
      success: "#55FFAD",
      warning: "#FFD337",
      error: "#FF3737",
      info: "#55C2FF",
    },
    fontFamily: {
      sans: ["Jost", "sans-serif"],
      serif: ["Instrument Serif", "serif"],
      mono: ["Darker Grotesque", "monospace"],
    },
    extend: {},
  },
  plugins: [],
};
