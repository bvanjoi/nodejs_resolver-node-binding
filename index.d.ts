/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export class ExternalObject<T> {
  readonly '': {
    readonly '': unique symbol
    [K: symbol]: T
  }
}
export interface RawResolverOptions {
  extensions?: Array<string>
  enforceExtension?: boolean | undefined | null
  alias?: Record<string, string | undefined | null>
  aliasFields?: Array<string>
  conditionNames?: Array<string>
  symlinks?: boolean
  descriptionFile?: string | undefined | null
  mainFiles?: Array<string>
  mainFields?: Array<string>
  modules?: Array<string>
  preferRelative?: boolean
  enableUnsafeCache?: boolean
}
export interface ResolverInternal {
  
}
export function create(options: RawResolverOptions): ExternalObject<ResolverInternal>
export interface ResolveResult {
  status: boolean
  path?: string
}
export function resolve(resolver: ExternalObject<ResolverInternal>, base_dir: string, id: string): {status: boolean, path?: string}
