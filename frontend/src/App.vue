<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
</script>

<template>
  <div class="wrapper">
    <nav>
      <RouterLink to="/">Home</RouterLink>
      <RouterLink to="/pastes">Pastes</RouterLink>
      <RouterLink to="/about">About</RouterLink>
    </nav>
  </div>
  <main>
    <RouterView />
  </main>
</template>

<style scoped>
.wrapper {
  background-color: #888888;
  position: absolute;
  width: 100%;
  height: 50px;
  top: 0;
  overflow: auto;
  z-index: 100;
}

main {
  padding-top: 50px;
}

nav {
  width: 100%;
  font-size: 20px;
  text-align: center;
  margin-top: 0.5rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a:first-of-type {
  border: 0;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}
</style>
