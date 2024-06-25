<template>
  <div display="none"></div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import bus from "vue3-eventbus";

export default {
  name: "FormStep4",
  data: () => ({}),
  methods: {
    async restore_tooth(token, label) {
      console.log("restore =>", token);
    },
  },
};
</script>