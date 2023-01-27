<template>
  <section class="flex flex-col items-center gap-10">
    <h1  class="text-2xl text-center text-white font-bold mt-10">
      Ranked List
    </h1>
    <VoteItem v-for="item in items" :key="item.id" :item="item" />
  </section>
</template>

<script>
  import VoteItem from "@/components/VoteItem.vue"
  import { isReady, rankedItems } from "@/lib/quadratic-voting"

  export default {
    name: "RankedList",
    components: {
      VoteItem
    },
    data() {
      return {
        items: []
      }
    },
    created() {
      // make sure app is started before attempting to retrive items
      const wait = async() => {
        if (isReady()) {
          this.items = await rankedItems()
        } else {
          setTimeout(wait, 100)
        }
      }
      wait()
    },
  }
</script>
