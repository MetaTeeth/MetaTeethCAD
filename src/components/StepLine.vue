<template>
  <v-card class="mx-auto floating-card bg-blue-grey-lighten-5" max-width="400"
    :style="{ left: currentX + 'px', top: currentY + 'px' }">
    <template v-slot:title>
      <span class="font-weight-black" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag">步骤</span>
    </template>
    <template v-slot:prepend>
      <v-icon icon="mdi-list-box-outline" @mousedown="startDrag" @mousemove="drag" @mouseup="endDrag"
        @mouseleave="endDrag" />
    </template>
    <v-card-text>
      <div class="font-weight-bold ms-1 mb-2">开始</div>

      <v-timeline align="start" density="compact">
        <v-timeline-item v-for="message in messages" :key="message.time" :dot-color="message.color" :icon="message.icon" size="x-small">
          <div class="mb-4">
            <div class="font-weight-normal">
              <strong>{{ message.from }}</strong>
            </div>

            <div>{{ message.message }}</div>
          </div>
        </v-timeline-item>
      </v-timeline>
    </v-card-text>
  </v-card>
</template>

<style>
.floating-card {
  position: absolute;
}
</style>

<script>
export default {
  name: "StepLine",
  data: () => ({
    startX: 870,
    startY: 20,
    currentX: 870,
    currentY: 20,
    dragging: false,
    messages: [
      {
        from: '输入牙列 / Input',
        message: '口扫模型（上颌+下颌+咬合）',
        color: 'green',
        icon: 'mdi-loading mdi-spin'
      },
      {
        from: '牙位分割 / Segementation',
        message: '上下颌配准+分割',
        color: 'grey-darken-1',
      },
      {
        from: '牙位选择 / Tooth Selection',
        message: '如果牙位分割不准确，需手动输入牙位形状',
        color: 'grey-darken-1',
      },
      {
        from: '牙齿修复（粗糙）/ Restoration (Rough)',
        message: 'Shape Emebedding',
        color: 'grey-darken-1',
      },
      {
        from: '形态微调 / Fine tunning',
        message: '适应咬合关系和临牙约束',
        color: 'grey-darken-1',
      },
      {
        from: '修复方案选择 / Tech Selection',
        message: '牙冠？嵌体？',
        color: 'grey-darken-1',
      },
      {
        from: '材料约束 / Material constraint',
        message: '光学+力学约束',
        color: 'grey-darken-1',
      },
      {
        from: '输出 / Output',
        message: '效果预览并输出',
        color: 'grey-darken-1',
      }
    ],
  }),
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
  }
}
</script>