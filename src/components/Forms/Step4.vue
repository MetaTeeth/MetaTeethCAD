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
      invoke("backend_restore_tooth", { token: token, label: label }).then((obj) => {
        bus.emit("add-obj-to-scene", { obj: obj, name: `autores_${token}` });
      });
    },
  },
};
</script>