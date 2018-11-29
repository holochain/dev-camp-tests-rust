// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape-catch');
const Container = require("@holochain/holochain-nodejs");

// instantiate an app from the DNA JSON bundle
const app = Container.instanceFromNameAndDna("app", "dist/bundle.json")

// activate the new instance
app.start()

const bonnittaAddress = "QmWyA4MpWazSQBEh7WLTLdHPFCUk31hbcacnJr87LCWR9T"

test('use the commit_entry function to add a person entry', (t) => {
  const result = app.call("people", "main", "add_person", { name: "Bonnitta" })
  t.equal(result.address, bonnittaAddress)
  t.end()
})

test('use the get_entry function to get a person entry', (t) => {
  const result = app.call("people", "main", "get_person", { address: bonnittaAddress })
  t.deepEqual(result, { value: '{"name":"Bonnitta"}', entry_type: 'person' })
  t.end()
})

test('use validation rules to ensure that a persons name is equal to or greater than 2 characters', (t) => {
  const result = app.call("people", "main", "add_person", { name: "B" })
  t.notEqual(result.error, undefined)
  t.end()
})

test('use the link_entries function to link two people entries', (t) => {
  const addResult = app.call("people", "main", "add_person", {
    name: "Vincenzo"
  })
  const result = app.call("people", "main", "link_people", {
    base: bonnittaAddress,
    target: addResult.address,
    tag: "is friends with"
  })
  t.equal(result.success, true)
  t.end()
})

test('use the get_links function to return people linked from Bonnitta', (t) => {
  const result = app.call("people", "main", "get_relationships", {
    address: bonnittaAddress,
    tag: "is friends with"
  })
  t.deepEqual(result, { addresses: [ "QmXWHWFiuNcz5mYGAVUJkU6jsLdybZc6ZKFykC5CoC8niZ" ] })
  t.end()
})
