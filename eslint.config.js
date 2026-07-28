import js from "@eslint/js";
import tseslint from "typescript-eslint";
export default tseslint.config(
  { ignores: ["dist/**", "src-tauri/**", "node_modules/**", "**/*.js"] },
  ...tseslint.configs.recommended,
  {
    rules: {
      "no-unused-vars": "off",
      "no-console": "off",
      "@typescript-eslint/no-unused-vars": "error",
    },
  },
);
