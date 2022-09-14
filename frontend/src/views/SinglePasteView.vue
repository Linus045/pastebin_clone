<template>
  <p>{{ title }}</p>
  <div class="textfield">
    <textarea v-model="body" readonly />
  </div>
</template>

<script setup lang="ts">

const props = defineProps({
  hash: {
    type: String,
    required: true,
  },
});
</script>

<script lang="ts">
import { defineComponent, defineProps } from "vue";
import type { PasteEntry } from "../types/PasteEntry";
import apiURL from "@/global";

export default defineComponent({
  data() {
    return {
      title: "" as string,
      body: "" as string,
      hash: "" as string,
    };
  },
  mounted() {
    fetch(`${apiURL}/paste/${this.$props.hash}`, {
      mode: "cors",
    })
      .then((res) => res.json())
      .then((data: PasteEntry) => {
        this.title = data.title;
        this.body = data.body;
        this.hash = data.hash;
      });
  },
});
</script>

<style>
.textfield {
  padding-top: 2rem;
  height: 600px;
}

.textfield textarea {
  border: 2px dotted red;
  min-height: 100px;
  min-width: 100%;
  max-width: 100%;
  width: 100%;
  height: 70%;
}
</style>
