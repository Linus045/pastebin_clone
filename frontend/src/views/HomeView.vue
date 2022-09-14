<template>
  <form @submit.prevent="on_create_paste">
    <div class="textfield">
      <input placeholder="Title" v-model="title" />
      <textarea placeholder="Enter code here..." v-model="body" />
      <button>Create Paste</button>
    </div>
  </form>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import apiURL from "@/global";

export default defineComponent({
  data() {
    return {
      title: "Title here" as string,
      body: "Your code here" as string,
    };
  },
  methods: {
    on_create_paste() {
      const request_body = JSON.stringify({
        title: this.title,
        body: this.body,
      });
      console.log(request_body);
      fetch(`${apiURL}/create`, {
        method: "post",
        mode: "cors",
        headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
          "Access-Control-Allow-Origin": "*",
        },
        body: request_body,
      })
        .then((response) => response.json())
        .then((response) => {
          console.log("Reponse: ", response);
          this.$router.push(`/paste/${response.hash}`);
        });
    },
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
