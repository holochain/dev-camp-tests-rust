// This test file uses the tape testing framework.
// To learn more, go here: https://github.com/substack/tape
const test = require('tape');
const Container = require('@holochain/holochain-nodejs');

// instantiate an app from the DNA JSON bundle
const app = Container.loadAndInstantiate("dist/bundle.json")

// activate the new instance
app.start()

test('commit', (t) => {
  const result = app.call("people", "main", "hash_post", JSON.stringify({ content: "test" }))
  t.equal(JSON.parse(result), "QmaErP4TAA7C39JfDFqzq3R57GnhevtoVC7H5eWK9r8hWU")
  t.end()
})
