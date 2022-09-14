<template>
  <div class="about">
    <h1>This is an about page</h1>
    Github: <a href="https://www.github.com/Linus045">Linus045</a>
    <!-- <p>Host URL: {{ hostURL }}</p> -->
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import apiURL from "@/global";
export default defineComponent({
  data() {
    return {
      hostURL: apiURL,
    };
  },
});
</script>

<!-- @media (min-width: 1024px) { -->
<!--   .about { -->
<!--     min-height: 100vh; -->
<!--     display: flex; -->
<!--     align-items: center; -->
<!--   } -->
<!-- } -->
<style>

</style>
