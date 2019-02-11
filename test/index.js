// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape-catch');
const { Config, Conductor } = require("../../../rust/nodejs_conductor");

// instantiate an alice from the DNA JSON bundle
const aliceName = "alice"
const dnaPath = "dist/bundle.json"
const instanceAlice = Config.instance(Config.agent(aliceName), Config.dna(dnaPath))
const config = Config.conductor([instanceAlice], { debugLog: false })
const conductor = new Conductor(config)
conductor.start()

const bonnittaAddress = "QmbL7tDsQumvsUTDVZo5mtJknhV6bT28yZDuTdyHQdfqTs"

test('use the commit_entry function to add a person entry', (t) => {
  let result
  try {
    result = conductor.call(aliceName, "people", "add_person", { name: "Bonnitta" })
  } catch (e) {}
  t.deepEqual(result, { Ok: bonnittaAddress })
  t.end()
})

test('use the get_entry function to get a person entry', (t) => {
  let result
  try {
    result = conductor.call(aliceName, "people", "get_person", { address: bonnittaAddress })
  } catch (e) {}
  t.deepEqual(result, { Ok: { App: [ 'person', '{"name":"Bonnitta"}' ] } })
  t.end()
})

test('use validation rules to ensure that a persons name is equal to or greater than 2 characters', (t) => {
  let result = {}
  try {
    result = conductor.call(aliceName, "people", "add_person", { name: "B" })
  } catch (e) {}
  t.notEqual(result.Err, undefined)
  t.end()
})

test('use the link_entries function to link two people entries', async (t) => {
  let addResult, result
  try {
    addResult = await conductor.callSync(aliceName, "people", "add_person", {
      name: "Vincenzo"
    })
    result = await conductor.callSync(aliceName, "people", "link_people", {
      base: bonnittaAddress,
      target: addResult.Ok,
      tag: "is friends with"
    })
  } catch (e) {}
  t.deepEqual(result, { Ok: null })
  t.end()
})

test('use the get_links function to return people linked from Bonnitta', (t) => {
  let result
  try {
    result = conductor.call(aliceName, "people", "get_relationships", {
      address: bonnittaAddress,
      tag: "is friends with"
    })
  } catch (e) {}
  t.deepEqual(result, { Ok: { addresses: [ "QmPcNictUVyk9tki1TwnsZ2RzzuPYdNPoFXZReRQLUJb4X" ] } })
  t.end()
  conductor.stop()
})