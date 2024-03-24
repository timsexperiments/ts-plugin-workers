import typeScriptEslint from '@typescript-eslint/eslint-plugin';
import parser from '@typescript-eslint/parser';
import prettierRecommended from 'eslint-config-prettier';

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
  {
    files: ['src/**/*.{ts,js}', 'eslint.config.mjs'],
    languageOptions: {
      parser,
      parserOptions: {
        project: './tsconfig.json',
      },
    },
    plugins: {
      '@typescript-eslint': typeScriptEslint,
    },
    rules: {
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', destructuredArrayIgnorePattern: '^_' },
      ],
      '@typescript-eslint/consistent-type-imports': [
        'error',
        { prefer: 'type-imports', fixStyle: 'inline-type-imports' },
      ],
    },
  },
  prettierRecommended,
];
