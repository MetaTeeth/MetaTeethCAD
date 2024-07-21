<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <!-- <ToolDial ref="tool_dial" />
    <StepLine ref="step_line" />
    <MeshPanel ref="mesh_panel" />

    <FormStep1 ref="form_step1" />
    <FormStep2 ref="form_step2" />
    <FormStep3 ref="form_step3" />
    <FormStep4 ref="form_step4" /> -->

    <ThreeScene ref="three_scene" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event";
import bus from "vue3-eventbus";
import ThreeScene from "../../../components/ThreeScene.vue";


export default {
  name: "DatasetProducer",
  components: {
    ThreeScene,
  },
  props: {},
  data() {
    return {};
  },
  computed: {},
  watch: {},

  methods: {
    init() { },
  },
  created() { },
  mounted() {
  },
  beforeCreate() { },
  beforeMount() { },
  beforeUpdate() { },
  updated() { },
  beforeDestroy() { },
  destroyed() { },
  activated() { },
};
</script>