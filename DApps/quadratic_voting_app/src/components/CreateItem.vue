<template>
  <!-- .prevent will prevent the page from redirecting -->
  <form @submit.prevent="submit" class="bg-gray-300 p-10 w-1/2 mx-auto rounded">
    <input 
      type="text"
      v-model="title"
      placeholder="Title"
      required
      class="px-3 py-1 w-full mb-10"
    /><br/>
    <input
      type="file"
      placeholder="Upload image"
      @input="uploadImage"
      required
      class="text-gray-300"
    /><br/>

    <p class="mb-10 text-gray-300" :class="imageHash ? 'mt-3' : ''">
      {{ imageHash || "" }}
    </p>
    <textarea
      v-model="description" 
      placeholder="Description"
      required
      class="px-3 py-1 w-full mb-10"
    /><br/>
    <input
      type="submit"
      value="Create Item"
      class="
        block
        mx-auto
        px-5
        py-3
        bg-blue-600
        hover:bg-blue-500
        text-white
        cursor-pointer
      "
    />
  </form>
</template>

<script>
  import { uploadFile, createItem } from "@/lib/quadratic-voting"
  
  export default {
    name: "CreateItem",
    methods: {
      async uploadImage(e) {
        const file = e.target.files[0] // get file from oninput event
        this.imageHash = await uploadFile(file) // upload to IPFS network
      },
      async submit() {
        await createItem(this.title, this.imageHash, this.description)
      }
    },
    data() {
      return {
        title: "",
        imageHash: null,
        description: ""
      }
    }
  }
</script>
