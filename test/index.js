
// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const { Config, Scenario } = require("@holochain/holochain-nodejs")
Scenario.setTape(require('tape-catch'))

const dnaPath = "./dist/bundle.json"
const agentAlice = Config.agent("alice")
const dna = Config.dna(dnaPath)
const instanceAlice = Config.instance(agentAlice, dna)
const scenario = new Scenario([instanceAlice])

const bonnittaAddress = "QmbL7tDsQumvsUTDVZo5mtJknhV6bT28yZDuTdyHQdfqTs"

scenario.runTape("use the commit_entry function to add a person entry", (t, { alice }) => {
  let result
  try {
    result = alice.call("people", "add_person", { name: "Bonnitta" })
  } catch (e) {}
  t.deepEqual(result, { Ok: bonnittaAddress })
})

scenario.runTape("use the get_entry function to get a person entry", (t, { alice }) => {
  let result
  try {
    alice.call("people", "add_person", { name: "Bonnitta" })
    result = alice.call("people", "get_person", { address: bonnittaAddress })
  } catch (e) {}
  t.deepEqual(result, { Ok: { App: [ 'person', '{"name":"Bonnitta"}' ] } })
})

scenario.runTape("use the get_entry function to get a person entry", (t, { alice }) => {
  let result
  try {
    alice.call("people", "add_person", { name: "Bonnitta" })
    result = alice.call("people", "get_person", { address: bonnittaAddress })
  } catch (e) {}
  t.deepEqual(result, { Ok: { App: [ 'person', '{"name":"Bonnitta"}' ] } })
})

scenario.runTape("use validation rules to ensure that a persons name is equal to or greater than 2 characters", (t, { alice }) => {
  let result = {}
  try {
    result = alice.call("people", "add_person", { name: "B" })
  } catch (e) {}
  t.notEqual(result.Err, undefined)
})

scenario.runTape("use the link_entries function to link two people entries", async (t, { alice }) => {
  let addResult, result
  try {
    addResult = await alice.callSync("people", "add_person", {
      name: "Vincenzo"
    })
    result = await alice.callSync("people", "link_people", {
      base: bonnittaAddress,
      target: addResult.Ok,
      tag: "is friends with"
    })
  } catch (e) {}
  t.deepEqual(result, { Ok: null })
})

scenario.runTape("use the get_links function to return people linked from Bonnitta", async (t, { alice }) => {
  let result
  try {
    // add Bonnitta
    await alice.callSync("people", "add_person", { name: "Bonnitta" })
    // add Vincenzo
    let addResult = await alice.callSync("people", "add_person", {
      name: "Vincenzo"
    })
    // make Bonnitta friends with Vincenzo
    await alice.callSync("people", "link_people", {
      base: bonnittaAddress,
      target: addResult.Ok,
      tag: "is friends with"
    })
    // get a list of the address of people who are friends with Bonnitta
    result = alice.call("people", "get_relationships", {
      address: bonnittaAddress,
      tag: "is friends with"
    })
  } catch (e) {}
  t.deepEqual(result, { Ok: { addresses: [ "QmPcNictUVyk9tki1TwnsZ2RzzuPYdNPoFXZReRQLUJb4X" ] } })
})