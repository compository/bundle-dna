import wasm from "./Cargo.toml";

export async function bundle_dna(...args) {
  const e = await wasm();
  return e.bundle_dna(...args);
}
