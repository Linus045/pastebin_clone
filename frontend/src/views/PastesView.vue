<template>
  <div v-if="itemsLoadingFailed">
    <p>Loading failed. Server might be offline.</p>
  </div>
  <div v-else>
    <table class="table">
      <th>Title</th>
      <th>Created</th>
      <th>Visited</th>
      <tr v-for="item in orderedItems" :key="item.hash" class="table-row">
        <td>
          <RouterLink :to="{ path: 'paste/' + item.hash }">
            {{ item.title }}
          </RouterLink>
        </td>
        <td width="15%">
          {{
          item.creation_date
          ? new Date(item.creation_date).toLocaleDateString()
          : "-"
          }}
        </td>
        <td width="5%" align="center">
          {{ item.click_count }}
        </td>
      </tr>
    </table>
  </div>
</template>

<script lang="ts">
import { RouterLink } from "vue-router";
import { defineComponent } from "vue";
import type { PasteEntry } from "../types/PasteEntry";
import apiURL from "@/global";

export default defineComponent({
  mounted() {
    fetch(`${apiURL}/pastes`, {
      mode: "cors",
    })
      .then((res) => res.json())
      .then((data: { pastes: PasteEntry[] }) => {
        this.items.push(...data.pastes);
      })
      .catch(() => {
        this.itemsLoadingFailed = true;
      });
  },
  data() {
    return {
      items: [] as PasteEntry[],
      itemsLoadingFailed: false as boolean,
    };
  },
  computed: {
    orderedItems() {
      const orderedItems = [...this.items];
      orderedItems.sort((a: PasteEntry, b: PasteEntry) => {
        const dateA = new Date(a.creation_date);
        const dateB = new Date(b.creation_date);
        let sort = dateA - dateB;
        if (sort == 0) sort = b.click_count - a.click_count;
        return sort;
      });
      return orderedItems;
    },
  },
});
</script>

<style>
.pastesview {
  background-color: #aaaaaa;
}

.table {
  padding-left: 0px;
  border-collapse: collapse;
}

.table th {
  padding-left: 0px;
  font-size: 120%;
  font-weight: 600;
}

.table td {
  border: 2px solid #aaaaaa;
}

.table-row {
  list-style: none;
  border: 1px solid #bbbbbb;
  padding-left: 0px;
}

.table-row a:hover {
  text-decoration: underline;
}
</style>
