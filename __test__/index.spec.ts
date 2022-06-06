import path from 'path'

import test from 'ava'

import factory from '../index'

test('sync function from native code', (t) => {
  const resolver = factory.create(JSON.stringify({}))
  const result = factory.resolve(resolver, __dirname, './lib.js')
  t.is(result.path, path.resolve(__dirname, './lib.js'))
  t.is(result.status, true)
})
