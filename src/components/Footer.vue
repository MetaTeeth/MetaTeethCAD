<template>
  <div>
    <v-footer
      class="bg-blue-grey-lighten-3 text-center d-flex flex-column"
      :style="autoHeight"
    >
      <div class="align-center text-subtitle-1">
        {{ new Date().getFullYear() }} — <strong>幻齿 CAD / BeauTee CAD</strong>
      </div>
    </v-footer>
  </div>
</template>

<script>
export default {
  name: "Footer",
  data() {
    return {
      autoHeight: {
        height: parseInt(window.innerHeight * 0.05) + "px",
      },
    };
  },
  mounted() {
    window.addEventListener("resize", () => {
      this.autoHeight.height = parseInt(window.innerHeight * 0.05) + "px";
    });
  },
};
</script>