import test from 'ava'

import datahike from '../index.js'

test('db works', (t) => {
  datahike.init()
  t.assert(true)
})
