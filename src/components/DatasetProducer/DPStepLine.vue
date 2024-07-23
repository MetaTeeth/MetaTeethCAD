<template>
  <v-card class="mx-auto floating-card" max-width="300" :style="{ left: currentX + 'px', top: currentY + 'px' }">
    <template v-slot:title>
      <span class="font-weight-black" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag">步骤</span>
    </template>
    <template v-slot:prepend>
      <v-icon icon="mdi-list-box-outline" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag" />
    </template>
    <v-card-text>
      <v-stepper-vertical class="font-weight-regular text-body-2" flat>


        <template v-slot:default="{ step }">
          <v-stepper-vertical-item :complete="step > 1" subtitle="Raw Inputs" title="原始数据输入" value="1">
            <v-text-field v-for="(hint, index) in rawInputHints" :key="index" density="compact" width="200" readonly
              prepend-inner-icon="mdi-upload" clearable @mousedown:control="clickAddRawInput(index)" @click:clear="clickRemoveRawInput(index)"
              v-model="rawInputs[index].name" variant="outlined" :label="hint" loading :disabled="rawInputs[index].loading">
              <template v-slot:loader>
                <v-progress-linear :active="rawInputs[index].loading" color="success" height="5" indeterminate />
              </template>
            </v-text-field>
            <template v-slot:next="{ next }">
              <v-btn color="primary" @click="next">下一步</v-btn>
            </template>
            <template v-slot:prev></template>
          </v-stepper-vertical-item>


          <v-stepper-vertical-item :complete="step > 2" subtitle="Auto Segmentation" title="牙位分割" value="2">
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, ratione debitis quis est labore
            voluptatibus!
            Eaque cupiditate minima, at placeat totam, magni doloremque veniam neque porro libero rerum unde voluptatem!

            <template v-slot:next="{ next }">
              <v-btn color="primary" @click="next">下一步</v-btn>
            </template>

            <template v-slot:prev="{ prev }">
              <v-btn variant="plain" @click="prev">上一步</v-btn>
            </template>
          </v-stepper-vertical-item>

          <v-stepper-vertical-item subtitle="Validation & Archive" title="验证与归档" value="3" @click:next="clickArchive()">
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Commodi, ratione debitis quis est labore
            voluptatibus!
            Eaque cupiditate minima, at placeat totam, magni doloremque veniam neque porro libero rerum unde voluptatem!

            <template v-slot:next="{ next }">
              <v-btn color="primary" text="Finish" @click="next">归档</v-btn>
            </template>

            <template v-slot:prev="{ prev }">
              <v-btn variant="plain" @click="prev">上一步</v-btn>
            </template>
          </v-stepper-vertical-item>
        </template>
      </v-stepper-vertical>
    </v-card-text>
  </v-card>
</template>

<style>
.floating-card {
  position: absolute;
}
</style>

<script setup>
import { VStepperVertical, VStepperVerticalItem } from "vuetify/labs/VStepperVertical";
</script>

<script>
import { open, save } from "@tauri-apps/api/dialog";
import bus from "vue3-eventbus";
import { getFileNameFromPath, getHashToken } from "@/scripts/utils";
import { APIRegister } from "@/scripts/APIs";
import { loadMeshUtil, exportPLY } from "@/scripts/MeshLoader";

export default {
  name: "DPStepLine",
  data: () => ({
    startX: 890,
    startY: 20,
    currentX: 890,
    currentY: 20,
    dragging: false,
    finished: false,
    rawInputs: [
      { name: null, filePath: null, loading: false, token: "" },
      { name: null, filePath: null, loading: false, token: "" },
      { name: null, filePath: null, loading: false, token: "" },
      { name: null, filePath: null, loading: false, token: "" },
    ],
    rawInputHints: ["上颌 / Upper Jaw", "下颌 / Lower Jaw", "咬合左侧 / Bite Left", "咬合右侧 / Bite Right"],
  }),
  mounted() {},
  methods: {
    startDrag(e) {
      this.dragging = true;
      this.startX = e.clientX - this.currentX;
      this.startY = e.clientY - this.currentY;
    },
    drag(e) {
      if (this.dragging) {
        this.currentX = e.clientX - this.startX;
        this.currentY = e.clientY - this.startY;
      }
    },
    endDrag() {
      this.dragging = false;
    },
    clickArchive() {
      this.finished = true
      alert('Finished')
    },
    async clickAddRawInput(pos) {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "mesh", extensions: ["obj", "stl", "ply"] }],
      });
      if (filePath != null) {
        this.rawInputs[pos].loading = true;
        this.rawInputs[pos].filePath = filePath;

        loadMeshUtil(
          filePath,
          object3D => {
            const bin = exportPLY(object3D);
            const local_token = getHashToken(bin);
            bus.emit('meta-teeth/new-mesh-added', { mesh: object3D, token: local_token });
            this.rawInputs[pos].loading = false;
            // APIRegister(
            //   bin,
            //   resp => {
            //     if (resp.status != 200) {
            //       console.error('[ERROR] <APIRegister>', resp.status);
            //       return;
            //     }
            //     // 注册完成 结束加载
            //     const token = resp.data.token;
            //     this.rawInputs[pos].token = token;
            //     this.rawInputs[pos].loading = false;

            //   },
            //   err => console.error('[ERROR] <APIRegister>', err)
            // );
          },
          () => {}, // process callback
          err => console.error('[ERROR] <loadMeshUtil>', err)
        );
      }
    },
    async clickUploadRawInputs() {
      console.log('?');
    },
    async clickRemoveRawInput(pos) {
    },
  },
};
</script>

<style scoped>
.font-mono {
  font-family: 'monospace';
}
</style>