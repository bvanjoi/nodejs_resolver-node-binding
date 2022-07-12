import path from 'path'
import test from 'ava'
import factory, { RawResolverOptions } from '../index'

test('sync function from native code', (t) => {
  const resolver = factory.create({})
  const result = factory.resolve(resolver, __dirname, './fixture/lib')
  t.is(result, path.resolve(__dirname, './fixture/lib.js'))
})

test('resolve do not exist file', (t) => {
  const resolver = factory.create({})
  let count = 0
  let encounterError = false
  try {
    count += 1
    factory.resolve(resolver, __dirname, './lib')
    count += 1
  } catch (err: any) {
    encounterError = true
    t.assert((err.message as string).includes('Resolve \'./lib\' failed in'))
  }
  t.is(count, 1)
  t.is(encounterError, true)
})

test('extensions options', (t) => {
  const resolverOptions: RawResolverOptions = {
    extensions: ['ts', 'js'],
  }
  const resolver = factory.create(resolverOptions)
  const result = factory.resolve(resolver, __dirname, './fixture/lib')
  t.is(result, path.resolve(__dirname, './fixture/lib.ts'))
  // with query and fragment
  const result2 = factory.resolve(resolver, __dirname, './fixture/lib?query#fragment')
  t.is(result2, path.resolve(__dirname, './fixture/lib.ts?query#fragment'))
})

test('alias options', (t) => {
  const resolverOptions: RawResolverOptions = {
    alias: [
      {
        key: '@alias',
        value: './fixture',
      },
      {
        key: '@false',
        value: undefined, // undefine -> None (represent `false`).
      }
    ]
  }
  const resolver = factory.create(resolverOptions)
  const result = factory.resolve(resolver, __dirname, '@alias/lib')
  t.is(result, path.resolve(__dirname, './fixture/lib.js'))

  const result2 = factory.resolve(resolver, __dirname, '@false/lib')
  t.is(result2, "false")
})

test('load sideeffects', (t) => {
  const resolver = factory.create({})
  const result = factory.loadSideEffects(resolver, path.resolve(__dirname, "./fixture/node_modules/a"));
  t.is(result?.boolVal, false)
  t.is(result?.arrayVal, undefined)
  t.is(result?.pkgFilePath, path.resolve(__dirname, "./fixture/node_modules/a/package.json"))
})

test("shared cache", (t) => {
  const sharedCache = factory.createExternalCache();
  const resolver1 = factory.createWithExternalCache({}, sharedCache);
  const resolver2 = factory.createWithExternalCache({}, sharedCache);

  const uncachedStart = process.hrtime.bigint();
  factory.loadSideEffects(resolver1, path.resolve(__dirname, "./fixture/node_modules/a"));
  const uncachedEnd = process.hrtime.bigint();
  const uncachedDuration = uncachedEnd - uncachedStart;

  const cachedStart = process.hrtime.bigint();
  factory.loadSideEffects(resolver2, path.resolve(__dirname, "./fixture/node_modules/a"));
  const cachedEnd = process.hrtime.bigint();
  const cachedDuration = cachedEnd - cachedStart;
  console.log('uncached: ', uncachedDuration, 'cached: ', cachedDuration);
  // maybe expose content in cache and ensure it is not empty may be a better choice.
  // but I think the following statement will usefully.
  t.is(cachedDuration - uncachedDuration < 0, true)
})
