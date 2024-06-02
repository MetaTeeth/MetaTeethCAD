<template>
  <v-container class="floating-tool" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag" @mouseleave="endDrag"
    :style="{ left: currentX + 'px', top: currentY + 'px' }">
    <v-fab class="me-4" color="surface-variant" icon="mdi-tooth-outline">
    </v-fab>
    <v-speed-dial location="right bottom" absolute transition="fade-transition" activator="parent">
      <v-btn key="1" @click="clickConsole()" color="indigo-lighten-1" icon="mdi-console" size="40"></v-btn>
      <v-btn key="2" color="indigo-lighten-1" icon="mdi-file-document-plus-outline" size="40"></v-btn>
      <v-btn key="3" color="indigo-lighten-1" icon="mdi-file-document-arrow-right-outline" size="40"></v-btn>
      <v-btn key="4" color="indigo-lighten-1" icon="mdi-cog-outline" size="40"></v-btn>
    </v-speed-dial>
  </v-container>
</template>

<style>
.floating-tool {
  position: absolute;
  padding: 0px;
  cursor: move;
  user-select: none;
  width: 50px;
  height: 47px;
}
</style>
<script>
import bus from 'vue3-eventbus';

export default {
  name: "ToolDial",
  data() {
    return {
      startX: 100,
      startY: 100,
      currentX: 100,
      currentY: 100,
      dragging: false
    };
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
    clickConsole() {
      bus.emit('click-tool-dial', { type: 'console' });
    }
  },
  mounted() {
  },
}
</script>