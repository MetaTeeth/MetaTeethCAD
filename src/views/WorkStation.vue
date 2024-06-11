<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <ToolDial ref="tool_dial" />
    <StepLine ref="step_line" />
    <FormStep1 ref="form_step1" />
    <ThreeScene ref="three_scene" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event"
import bus from 'vue3-eventbus';
import ThreeScene from "../components/ThreeScene.vue";
import ToolDial from "../components/ToolDial.vue";
import StepLine from "../components/StepLine.vue";
import FormStep1 from "../components/Forms/Step1.vue";

export default {
  name: 'WorkStation',
  components: { ThreeScene, FormStep1, ToolDial, StepLine },
  props: {},
  data() {
    return {
      state: 1
    };
  },
  computed: {},
  watch: {},

  methods: {
    init() {
      bus.on("click-tool-dial", this.handleToolDial);
    },
    handleToolDial(param) {
      switch (param.type) {
        case "console":
          this.$refs.form_step1.visible = true;
          break;
        default:
          break;
      }
      // this.$refs.step_line.messages[this.state - 1]["current"] = false;
      // this.state += 1;
      // this.$refs.step_line.messages[this.state - 1]["current"] = true;
    },

  },
  created() {
  },
  mounted() {
    this.init();
  },
  beforeCreate() { },
  beforeMount() { },
  beforeUpdate() { },
  updated() { },
  beforeDestroy() { },
  destroyed() { },
  activated() { },
}
</script>