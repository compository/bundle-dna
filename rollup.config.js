import rust from "@wasm-tool/rollup-plugin-rust";

export default {
  input: `index.js`,
  output: [{ dir: "dist", format: "es", sourcemap: true }],
  plugins: [
    rust({
      inlineWasm: true,
    }),
  ],
};
