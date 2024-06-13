<template>
  <v-dialog v-model="visible" max-width="700">
  </v-dialog>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
import bus from 'vue3-eventbus';

var color_circles = {
  0: "rgb(230, 230, 230)",
  11: "#628bd5", 12: "#acb839", 13: "#6a70d7", 14: "#70c563", 15: "#d564bb", 16: "#5aca7b", 17: "#882f7c", 18: "#86ac40",
  21: "#503788", 22: "#d1972c", 23: "#b87dd3", 24: "#3f7f25", 25: "#cf4c84", 26: "#4cbe84", 27: "#842757", 28: "#43c29e",
  31: "#d85a49", 32: "#36dee6", 33: "#c44d55", 34: "#397a3a", 35: "#d77eb6", 36: "#9fbd6c", 37: "#bc4d66", 38: "#cbaa50",
  41: "#953f26", 42: "#777022", 43: "#bd6b27", 44: "#ce9157", 45: "#9162b0", 46: "#df8a3c", 47: "#742f8f", 48: "#5699e5",
};

export default {
  name: "FormStep2",
  data: () => ({
    visible: false,
    labels_store: {} // { name(token): labels }
  }),
  methods: {
    async segment_jaw(token, jaw_kind) {
      invoke('backend_segment_jaw', { token: token, jawkind: jaw_kind }).then((ret) => {
        const labels = ret.labels;
        this.labels_store[token] = labels;
        var vert_colors = []

        for (var ind = 0; ind < labels.length; ++ind) {
          vert_colors.push(color_circles[labels[ind]]);
        }

        bus.emit("change-vertex-color", { name: token, colormap: vert_colors });

        bus.emit("go-to-step", { step: 3 });
      });
    }
  }
}

</script>