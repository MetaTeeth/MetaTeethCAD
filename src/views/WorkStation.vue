<template>
  <v-main class="d-flex align-center justify-center flex-wrap">
    <ToolDial ref="tool_dial" />
    <StepLine ref="step_line" />
    <ThreeScene ref="three_scene" />
    <SubmmitForm ref="submmit_form" />
  </v-main>
</template>

<script>
import { listen } from "@tauri-apps/api/event"
import ThreeScene from "../components/ThreeScene.vue";
import SubmmitForm from "../components/SubmmitForm.vue";
import ToolDial from "../components/ToolDial.vue";
import StepLine from "../components/StepLine.vue";


export default {
  name: 'WorkStation',
  components: { ThreeScene, SubmmitForm, ToolDial, StepLine },
  props: {},
  data() {
    return {};
  },
  computed: {},
  watch: {},

  methods: {
    init() {
      listen('tauri://file-drop', (event) => {
        const filePath = event['payload'][0];
        console.log(filePath);
        this.$refs.three_scene.load_OBJ(filePath);
        this.$refs.submmit_form.visible = true;
      });
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