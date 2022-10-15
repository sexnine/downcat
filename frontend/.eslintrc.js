module.exports = {
  root: true,
  env: {
    browser: true,
    es2021: true,
    node: true,
  },
  extends: [
    "plugin:vue/vue3-recommended",
    "@vue/typescript/recommended",
    "prettier",
    // "eslint:recommended"
  ],
  parserOptions: {
    ecmaVersion: 2021,
  },
  plugins: ["prettier", "vue"],
  rules: {
    "vue/component-definition-name-casing": ["warn", "kebab-case"],
    "linebreak-style": "off",
    "vue/script-setup-uses-vars": "error",
    "@typescript-eslint/ban-ts-comment": "off",
    "prettier/prettier": [
      "warn",
      {
        singleQuote: false,
        semi: true,
        vueIndentScriptAndStyle: true,
        endOfLine: "crlf",
        bracketSameLine: false,
      },
    ],
  },
};
