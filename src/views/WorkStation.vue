<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <ToolDial ref="tool_dial" />
    <StepLine ref="step_line" />
    <MeshPanel ref="mesh_panel" />

    <FormStep1 ref="form_step1" />
    <FormStep2 ref="form_step2" />
    <FormStep3 ref="form_step3" />
    <FormStep4 ref="form_step4" />

    <ThreeScene ref="three_scene" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event";
import bus from "vue3-eventbus";
import ThreeScene from "../components/ThreeScene.vue";
import ToolDial from "../components/ToolDial.vue";
import StepLine from "../components/StepLine.vue";
import MeshPanel from "../components/MeshPanel.vue";
import FormStep1 from "../components/Forms/Step1.vue";
import FormStep2 from "../components/Forms/Step2.vue";
import FormStep3 from "../components/Forms/Step3.vue";
import FormStep4 from "../components/Forms/Step4.vue";

export default {
  name: "WorkStation",
  components: {
    ThreeScene,
    FormStep1,
    FormStep2,
    FormStep3,
    FormStep4,
    ToolDial,
    StepLine,
    MeshPanel,
  },
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
          } else if (this.state == 3) {
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
      } else if (this.state == 4) {
        const params = this.$refs.form_step3.pickedTooth;
        for (const pack of params) {
          if (pack.sel_datasource == "自动分割") {
            await this.$refs.form_step4.restore_tooth(`autoseg_${pack.id}`, pack.id);
          }
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