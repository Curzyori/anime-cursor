/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#FF6B9D",
          active: "#E05580",
          muted: "rgba(255,107,157,0.15)",
        },
        accent: { purple: "#C44AFF" },
        canvas: {
          dark: "#0F0F23",
          "dark-soft": "#13132A",
        },
        surface: {
          DEFAULT: "#16162A",
          elevated: "#1E1E38",
          card: "#25254A",
          hover: "#2E2E55",
        },
        ink: {
          DEFAULT: "#1A1A2E",
          body: "#E8E8F0",
          soft: "#9B9BB5",
          muted: "#6B6B85",
          faint: "#4A4A65",
        },
        semantic: {
          success: "#4ADE80",
          error: "#FF4D6D",
          warning: "#FBBF24",
          info: "#60A5FA",
        },
        hairline: {
          DEFAULT: "#2E2E55",
          soft: "#222244",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        code: ["JetBrains Mono", "Fira Code", "monospace"],
      },
      spacing: {
        section: "48px",
      },
      borderRadius: {
        xs: "4px",
        sm: "6px",
        md: "8px",
        lg: "12px",
        xl: "16px",
        xxl: "24px",
      },
      transitionTimingFunction: {
        "snap-out": "cubic-bezier(0.16, 1, 0.3, 1)",
      },
      transitionDuration: {
        "200": "200ms",
        "300": "300ms",
      },
      keyframes: {
        "slide-in-right": {
          from: { transform: "translateX(20px)", opacity: "0" },
          to: { transform: "translateX(0)", opacity: "1" },
        },
        "fade-out": {
          from: { opacity: "1" },
          to: { opacity: "0" },
        },
        "pulse-glow": {
          from: { boxShadow: "inset 0 0 20px rgba(255,107,157,0.1)" },
          to: { boxShadow: "inset 0 0 40px rgba(255,107,157,0.2)" },
        },
      },
      animation: {
        "slide-in-right": "slide-in-right 250ms cubic-bezier(0.16,1,0.3,1)",
        "fade-out": "fade-out 300ms ease-out forwards",
        "pulse-glow": "pulse-glow 1s infinite alternate",
      },
    },
  },
  plugins: [],
};
