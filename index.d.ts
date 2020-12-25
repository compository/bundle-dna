/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} name
 * @param {string} uuid
 * @param {any} properties
 * @param {any} zomes
 * @param {any} wasms
 * @returns {any}
 */
export function bundle_dna(
  name: string,
  uuid: string,
  properties: any,
  zomes: any,
  wasms: any
): Promise<DnaFile>;

export interface DnaFile {
  dna: {
    content: {
      name: String;
      uuid: String;
      properties: Array<number>;
      zomes: Array<[string, { wasm_hash: Array<number> }]>;
    };
    hash: Array<number>;
  };
  code: Array<WasmCode>;
}

export type WasmCode = [Array<number>, { code: Array<number> }];
