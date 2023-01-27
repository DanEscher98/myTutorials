const QuadraticVoting = artifacts.require("QuadraticVoting")

contract("QuadraticVoting", (accounts) => {
  describe("deployment", () => {
    it("should be a valid contract address", () =>
      QuadraticVoting.deployed()
        .then((instance) => instance.address)
        .then((address) => {
          assert.notEqual(address, null)
          assert.notEqual(address, 0x0)
          assert.notEqual(address, "")
          assert.notEqual(address, undefined)
        }))
  })

  describe("items", () => {
    it("should be the correct item data", () => {
      let instance

      QuadraticVoting.deployed()
        .then((i) => (instance = i))
        .then(() =>
          instance.createItem(
            web3.utils.utf8ToHex("Chewbacca"), // title
            "ipfs_hash", // imageHash
            "The ultimate furry", // description
          ))
        .then(() => instance.itemCount())
        .then((count) => assert.equal(count, 1)) // should be 1 item
        .then(() => instance.items(0))
        .then((item) => {
          assert.equal(web3.utils.hexToUtf8(item.title), "Chewbacca")
          assert.equal(item.imageHash, "ipfs_hash")
          assert.equal(item.description, "The ultimate furry")
        })
    })
  })
})
