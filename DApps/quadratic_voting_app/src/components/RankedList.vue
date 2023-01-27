<template>
  <section class="flex flex-col proposals-center gap-10">
    <h1  class="text-2xl text-center text-white font-bold mt-10">
      Ranked List
    </h1>
    <VoteProposal v-for="proposal in proposals" :key="proposal.id" :proposal="proposal" />
  </section>
</template>

<script>
  import VoteProposal from "@/components/VoteProposal.vue"
  import { isReady, rankedProposals } from "@/lib/quadratic-voting"

  export default {
    name: "RankedList",
    components: {
      VoteProposal
    },
    data() {
      return {
        proposals: []
      }
    },
    created() {
      // make sure app is started before attempting to retrive proposals
      const wait = async() => {
        if (isReady()) {
          this.proposals = await rankedProposals()
        } else {
          setTimeout(wait, 100)
        }
      }
      wait()
    },
  }
</script>
