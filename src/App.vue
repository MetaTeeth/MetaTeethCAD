<template>
  <v-app>
    <router-view />
    <Footer />
  </v-app>
</template>

<script>
import Footer from "./components/Footer.vue";

export default {
  data() {
    return {};
  },
  components: { Footer },
};
</script>

<style>
/* 仅隐藏滚动条但允许滚动内容 */
body, html {
  overflow-y: scroll; /* 确保内容可以垂直滚动 */
  -ms-overflow-style: none; /* Internet Explorer 10+ */
  scrollbar-width: none; /* Firefox */
}

body::-webkit-scrollbar, html::-webkit-scrollbar {
  display: none; /* Chrome, Safari, Opera */
}
</style>