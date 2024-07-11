<template>
  <v-card class="mx-auto floating-card bg-blue-grey-lighten-5" max-width="600"
    :style="{ left: currentX + 'px', top: currentY + 'px' }">
    <template v-slot:title>
      <span class="font-weight-black" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag">资源面板</span>
    </template>
    <template v-slot:prepend>
      <v-icon icon="mdi-list-box-outline" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag" />
    </template>
    <v-container fluid>
      <v-row v-for="ctrl in controls" :key="ctrl.token" no-gutters align="center" justify="space-between">
        <v-col cols="4">
          <v-btn :icon="ctrl.visible ? `mdi-eye` : `mdi-eye-closed`" variant="text" color="black"
            @click="toggleVisibility(ctrl)"></v-btn>
        </v-col>
        <v-col>
          <v-chip link class="ma-2" :color="ctrl.picked ? `pink` : `primary`" @click="ctrl.picked = !ctrl.picked">
            {{ ctrl.name }}
          </v-chip>
        </v-col>
      </v-row>
    </v-container>
  </v-card>
</template>

<style>
.floating-card {
  position: absolute;
}
</style>

<script>
import bus from "vue3-eventbus";

const ShadeMode = {
  Pointcloud: 0,
  Wireframe: 1,
  Solid: 2
}

export default {
  name: "MeshPanel",
  data: () => ({
    startX: 90,
    startY: 20,
    currentX: 90,
    currentY: 20,
    dragging: false,
    controls: []
  }),
  mounted() {
    bus.on("add-obj-to-scene", (param) => {
      this.controls.push({
        token: param.token, name: param.name, visible: true
      })
    });
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
    toggleVisibility(obj) {
      obj.visible = !obj.visible;
      bus.emit("change-mesh-visibility", { name: obj.token, visible: obj.visible })
    }
  }
}
</script>