<template>
  <main class="p-10 min-h-screen bg-gray-900">
    <CreateItem />
    <RankedList />
  </main>
</template>

<script>
  import CreateItem from "@/components/CreateItem.vue"
  import RankedList from "@/components/RankedList.vue"

  export default {
    name: "App",
    components: {
      CreateItem,
      RankedList
    }
  }
</script>

<style lang="postcss" global>
@tailwind base;
@tailwind components;
@tailwind utilities;
</style>
