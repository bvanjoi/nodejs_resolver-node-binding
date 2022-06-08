import path from 'path'

import test from 'ava'

import factory, { RawResolverOptions } from '../index'

test('sync function from native code', (t) => {
  const resolver = factory.create({})
  const result = factory.resolve(resolver, __dirname, './fixture/lib')
  t.is(result.path, path.resolve(__dirname, './fixture/lib.js'))
  t.is(result.status, true)
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
  t.is(result.path, path.resolve(__dirname, './fixture/lib.ts'))
  t.is(result.status, true)
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
  t.is(result.path, path.resolve(__dirname, './fixture/lib.js'))
  t.is(result.status, true)

  const result2 = factory.resolve(resolver, __dirname, '@false/lib')
  t.is(result2.path, undefined)
  t.is(result2.status, false)
})
