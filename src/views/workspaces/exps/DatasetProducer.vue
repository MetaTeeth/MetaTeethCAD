<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <DPStepLine ref="dp_step_line" />

    <ThreeScene ref="three_scene" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event";
import bus from "vue3-eventbus";
import ThreeScene from "@/components/ThreeScene.vue";
import DPStepLine from "@/components/DatasetProducer/DPStepLine.vue";


export default {
  name: "DatasetProducer",
  components: {
    ThreeScene, DPStepLine
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