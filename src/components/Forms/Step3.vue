<template>
  <v-dialog v-model="visible" max-width="700">
    <v-card prepend-icon="mdi-account" title="缺损牙位选择">
      <v-card-text>
        <v-row dense> </v-row>
      </v-card-text>

      <v-card-text>
        <PreviewScene />
      </v-card-text>
      <v-divider></v-divider>

      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn text="取消" variant="plain" @click="visible = false"></v-btn>

        <v-btn
          color="primary"
          text="确认"
          variant="tonal"
          @click="confirm()"
        ></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import bus from "vue3-eventbus";
import PreviewScene from "../PreviewScene.vue";

export default {
  name: "FormStep3",
  components: { PreviewScene },
  data: () => ({
    visible: false,
  }),
  methods: {
    confirm() {
      this.visible = false;

      bus.emit("go-to-step", { step: 2 });
    },
  },
  created() {},
  //生命周期 - 挂载完成（可以访问DOM元素）
  mounted() {},
};
</script>