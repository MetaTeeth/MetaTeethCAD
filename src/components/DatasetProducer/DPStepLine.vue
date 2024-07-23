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
            <DPStep1 ref="step1" />
            <template v-slot:next="{ next }">
              <v-btn color="primary" @click="goNextStep(step, next)" :disabled="!canGoNextFuncs[step-1]()">下一步</v-btn>
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
import { open } from "@tauri-apps/api/dialog";
import { getFileNameFromPath, getHashToken } from "@/scripts/utils";
import { APIRegister } from "@/scripts/APIs";
import { loadMeshUtil, exportPLY } from "@/scripts/MeshLoader";
import bus from "vue3-eventbus";
import DPStep1 from "@/components/DatasetProducer/DPStep1.vue";

export default {
  name: "DPStepLine",
  components: { DPStep1 },
  data: () => ({
    startX: 890,
    startY: 20,
    currentX: 890,
    currentY: 20,
    dragging: false,
    finished: false,
    canGoNextFuncs: [() => true, () => true, () => true],
    goNextFuncs: [() => {}, () => {}, () => {}],
  }),
  mounted() {
    this.$nextTick(() => {
      this.canGoNextFuncs[0] = this.$refs.step1.canGoNext;
      this.goNextFuncs[0] = this.$refs.step1.clickUploadRawInputs;
    })
  },
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
    async goNextStep(step, next) {
      // call next when ready
      await this.goNextFuncs[step - 1](next);
    },
  },
};
</script>

<style scoped>
.font-mono {
  font-family: 'monospace';
}
</style>