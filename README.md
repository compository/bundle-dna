# Bundle-Dna

This is a small wasm package to bundle holochain DNAs from zome wasm codes, in the browser itself.

The code for this package is in rust but it's distributed as an npm package, ready to be installed in your js app.

## Installing

```bash
npm i https://github.com/compository/bundle-dna#build
```

## Usage

```js
import { bundle_dna, DnaFile } from 'bundle-dna';

...

// Bundle the dna
const dnaFile: DnaFile = await bundle_dna(
  name,
  uuid,
  properties,
  zomesDefs,
  codes
);
```

Take into account that this can take a lot of time, depending on how big and how many the zomes are.

## Building

```bash
npm i
npm run build
```

## Publishing to the build branch

```bash
npm run publish-to-branch
```
