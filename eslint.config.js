import path from 'node:path'
import globals from 'globals'
import eslint from '@eslint/js'
import tseslint from 'typescript-eslint'
import { fileURLToPath } from 'node:url'
import { includeIgnoreFile } from '@eslint/compat'
import eslingPluginSvelte from 'eslint-plugin-svelte'
import eslintPluginPrettierRecommended from 'eslint-plugin-prettier/recommended'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const gitignorePath = path.resolve(__dirname, '.gitignore')

export default [
    includeIgnoreFile(gitignorePath),
    eslint.configs.recommended,
    ...tseslint.configs.recommended,
    ...eslingPluginSvelte.configs['flat/prettier'],
    /**
     * js规则
     */
    {
        files: ['**/*.{js,mjs,cjs,ts,mts,cts,svelte}'],
        rules: {
            'no-console': 'error',
        },
    },
    /**
     * 配置全局变量
     */
    {
        languageOptions: {
            globals: {
                ...globals.browser,
                ws: true,
            },
        },
    },
    /**
     * svelte规则
     */
    {
        plugins: {
            svelte: eslingPluginSvelte,
        },
        files: ['**/*.svelte', '*.svelte'],
        languageOptions: {
            parserOptions: {
                parser: tseslint.parser,
                // parser: 'svelte-eslint-parser',
                ecmaVersion: 'latest',
                ecmaFeatures: {
                    jsx: false,
                },
            },
        },
        rules: {
            'no-undef': 'off',
        },
    },
    /**
     * prettier 配置
     */
    eslintPluginPrettierRecommended,
]
