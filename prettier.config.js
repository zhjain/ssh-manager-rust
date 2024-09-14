export default {
    arrowParens: 'avoid',
    endOfLine: 'auto',
    semi: false,
    singleQuote: true,
    svelteSortOrder: 'options-scripts-markup-styles',
    svelteStrictMode: true,
    svelteBracketNewLine: false,
    svelteAllowShorthand: true,
    plugins: ['prettier-plugin-svelte'],
    tabWidth: 4,
    overrides: [
        {
            files: '*.svelte',
            options: {
                singleQuote: false,
            },
        },
    ],
}
