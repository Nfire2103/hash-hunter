module.exports = {
  '**/*.{js,ts, jsx, tsx}': (filenames) => [
    `npx prettier --write ${filenames.map((filename) => `"${filename}"`).join(' ')}`,
    `npx eslint --fix ${filenames.map((filename) => `"${filename}"`).join(' ')}`,
  ],
  '**/*.{md,json}': (filenames) =>
    `npx prettier --write ${filenames.map((filename) => `"${filename}"`).join(' ')}`,
};
