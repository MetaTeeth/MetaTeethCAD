<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <ToolDial ref="tool_dial" />
    <StepLine ref="step_line" />

    <FormStep1 ref="form_step1" />
    <FormStep2 ref="form_step2" />
    <FormStep3 ref="form_step3" />

    <ThreeScene ref="three_scene" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event";
import bus from "vue3-eventbus";
import ThreeScene from "../components/ThreeScene.vue";
import ToolDial from "../components/ToolDial.vue";
import StepLine from "../components/StepLine.vue";
import FormStep1 from "../components/Forms/Step1.vue";
import FormStep2 from "../components/Forms/Step2.vue";
import FormStep3 from "../components/Forms/Step3.vue";

export default {
  name: "WorkStation",
  components: { ThreeScene, FormStep1, FormStep2, FormStep3, ToolDial, StepLine },
  props: {},
  data() {
    return {
      state: 1,
    };
  },
  computed: {},
  watch: {},

  methods: {
    init() {
      bus.on("click-tool-dial", this.handleToolDial);
      bus.on("go-to-step", this.handleMoveToStep);
    },
    handleToolDial(param) {
      switch (param.type) {
        case "console":
          if (this.state == 1) {
            this.$refs.form_step1.visible = true;
          }
          else if (this.state == 3) {
            this.$refs.form_step3.visible = true;
          }
          break;
        default:
          break;
      }
    },
    async handleMoveToStep(param) {
      if (param.step == this.state) return;

      this.$refs.step_line.markStepFinish(this.state);
      this.state = param.step;
      this.$refs.step_line.markStepCurrent(this.state);

      if (this.state == 2) {
        // segmentation
        const tokens = this.$refs.form_step1.tokens;

        if (tokens[0].length > 16) {
          await this.$refs.form_step2.segment_jaw(tokens[0], "upper");
        }
        if (tokens[1].length > 16) {
          await this.$refs.form_step2.segment_jaw(tokens[1], "lower");
        }
      }
    },
  },
  created() {},
  mounted() {
    this.init();
  },
  beforeCreate() {},
  beforeMount() {},
  beforeUpdate() {},
  updated() {},
  beforeDestroy() {},
  destroyed() {},
  activated() {},
};
</script>