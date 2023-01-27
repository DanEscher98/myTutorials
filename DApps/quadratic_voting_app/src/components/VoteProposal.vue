<template>
  <div class="bg-gray-300 p-10 rounded flex proposals-center">
    <img
      :src="`https://ipfs.io/ipfs/${proposal.imageHash}`"
      alt="proposal image"
      class="w-1/12 mr-10 rounded shadow"
    />
    <div class="mr-5">
      <button
        @click="upvote"
        class="
          text-xl
          px-4
          py-3
          border border-b-2 border-black
          rounded
          hover:bg-black hover:text-white
        "
      >
        &uarr;
      </button>
      <p class="text-lg text-center mt-2">{{ proposal.positiveWeight }}</p>
    </div>
    <div class="mr-10">
      <button
        @click="downvote"
        class="
          text-xl
          px-4
          py-3
          border border-b-2 border-black
          rounded
          hover:bg-black hover:text-white
        "
      >
        &darr;
      </button>
      <p class="text-lg text-center mt-2">{{ proposal.negativeWeight }}</p>
    </div>
    <div>
      <h2 class="text-xl font-bold mb-5">{{ proposal.title }}</h2>
      <p class="text-gray-600 mb-5">{{ proposal.description }}</p>
    </div>
    <div v-if="address() === proposal.owner" class="ml-auto">
      <p class="mb-5 text-center">
        Amount: {{ proposal.amount / 1_000_000_000 }} gwei
      </p>
      <button
        @click="claimGwei"
        class="block mx-auto px-5 py-3 bg-blue-600 hover:bg-blue-500 text-white"
      >
        Claim
      </button>
    </div>
    <div v-else-if="weight !== startWeight" class="ml-auto">
      <p class="mb-5 text-center">Weight: {{ weight }}</p>
      <p class="mb-5 text-center">Cost: {{ cost / 1_000_000_000 }} gwei</p>
      <button
        @click="submitVote"
        class="block mx-auto px-5 py-3 bg-blue-600 hover:bg-blue-500 text-white"
      >
        Submit Vote
      </button>
    </div>
  </div>
</template>

<script>
  import {
    address,
    currentWeight,
    calcCost,
    positiveVote,
    negativeVote,
    claim
  } from "@/lib/quadratic-voting"

  export default {
    name: "VoteProposal",
    props: ["proposal"],
    methods: {
      address,
      upvote() {
        this.weight += 1
        this.setCost()
      },
      downvote() {
        this.weight -= 1
        this.setCost()
      },
      async setCost() {
        if (this.weight === 0) {
          this.cost = 0
        } else {
          const isPositive = this.weight > 0
          const currWeight = await currentWeight(this.proposal.id, isPositive)
          this.cost = await calcCost(currWeight, Math.abs(this.weight))
        }
      },
      async submitVote() {
        if (this.weight >= 0) {
          // submit psoitive vote if weight is positive
          await positiveVote(this.proposal.id, this.weight, this.cost)
        } else if (this.weight < 0) {
          // submit negative vote if weight is negative
          await negativeVote(this.proposal.id, -this.weight, this.cost)
        }
      },
      async claimGwei() {
        await claim(this.proposal.id) // transfers rewards to owner wallet
      }
    },
    data() {
      return {
        weight: 0,
        startWeight: 0,
        cost: 0
      }
    },
    created() {
      const getWeight = async () => {
        // calculate the net weight to be used with voting controls
        const posWeight = await currentWeight(this.proposal.id, true)
        const negWeight = await currentWeight(this.proposal.id, false)
        this.weight = posWeight - negWeight
        // keep track of the weight we started with
        this.startedWeight = this.weight
      }
      getWeight()
    }
  }
</script>
