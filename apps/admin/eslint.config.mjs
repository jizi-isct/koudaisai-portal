import nextEslintPluginNext from "@next/eslint-plugin-next";
import nx from "@nx/eslint-plugin";
import importPlugin from "eslint-plugin-import";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import baseConfig from "../../eslint.config.mjs";

export default [
    { plugins: { "@next/next": nextEslintPluginNext } },
    ...baseConfig,
    ...nx.configs["flat/react-typescript"],
    {
        ignores: [
            ".next/**/*",
            "**/out-tsc",
            "out/**/*"
        ]
    },
    {
        files: ["**/*.ts", "**/*.tsx"],
        plugins: {
            "react-hooks": reactHooksPlugin,
            "import": importPlugin,
        },
        rules: {
            // 未使用変数（_プレフィクスは無視）
            "no-unused-vars": "off",
            "@typescript-eslint/no-unused-vars": ["warn", {
                argsIgnorePattern: "^_",
                varsIgnorePattern: "^_",
                caughtErrorsIgnorePattern: "^_",
            }],
            // 非nullアサーション・any禁止
            "@typescript-eslint/no-non-null-assertion": "error",
            "@typescript-eslint/no-explicit-any": "warn",
            // React Hooks
            "react-hooks/rules-of-hooks": "error",
            "react-hooks/exhaustive-deps": "warn",
            // インポート順
            "import/order": ["warn", {
                groups: ["builtin", "external", "internal", "parent", "sibling", "index"],
                alphabetize: { order: "asc", caseInsensitive: true },
            }],
        }
    }
];
