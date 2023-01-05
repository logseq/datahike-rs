/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function init(): void
export function databaseExists(config: string): boolean
export function createDatabase(config: string): void
export function deleteDatabase(config: string): void
export function transact(config: string, txData: string): string
/** inputs: [[type, edn]] e.g. [["db", config]] */
export function query(queryEdn: string, inputs: Array<[string, string]>): string
export function pull(inputDb: string, selector: string, eid: number | string): string
/**
 * eids: [1 2 3 4]
 * lookup_refs: []
 */
export function pullMany(inputDb: string, selector: string, eids: string): string
export function entity(inputDb: string, eid: number | string): string
/** index_edn: :avet, :aevt, :eavt */
export function datoms(inputDb: string, indexEdn: string): string
export function schema(inputDb: string): string
export function reverseSchema(inputDb: string): string
