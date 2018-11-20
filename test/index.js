// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('use the commit_entry function to add a person entry', (t) => {
  const result = app.call("people", "main", "add_person", JSON.stringify({ name: "Bonnitta" }))
  t.equal(JSON.parse(result).address, "QmWyA4MpWazSQBEh7WLTLdHPFCUk31hbcacnJr87LCWR9T")
  t.end()
})

// test('use the get_entry function to get a person entry', (t) => {
//   const result = app.call("people", "main", "add_person", JSON.stringify({ name: "Bonnitta" }))
//   t.equal(JSON.parse(result), "QmaErP4TAA7C39JfDFqzq3R57GnhevtoVC7H5eWK9r8hWU")
//   t.end()
// })

test('use validation rules to ensure that a persons name is equal to or greater than 2 characters', (t) => {
  const result = app.call("people", "main", "add_person", JSON.stringify({ name: "B" }))
  t.notEqual(JSON.parse(result).error, undefined)
  t.end()
})

// test('use the link_entries function to link two people entries', (t) => {
//   const result = app.call("people", "main", "add_person", JSON.stringify({ name: "Bonnitta" }))
//   t.equal(JSON.parse(result), "QmaErP4TAA7C39JfDFqzq3R57GnhevtoVC7H5eWK9r8hWU")
//   t.end()
// })

// test('use the get_links function to return people linked from Bonnitta', (t) => {
//   const result = app.call("people", "main", "add_person", JSON.stringify({ name: "Bonnitta" }))
//   t.equal(JSON.parse(result), "QmaErP4TAA7C39JfDFqzq3R57GnhevtoVC7H5eWK9r8hWU")
//   t.end()
// })