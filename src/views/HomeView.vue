<template>
  <v-main>
    <v-container>
      <v-row no-gutters>
        <v-col align="center">
          <v-btn @click="btnToSpace('/space/enamel_designer')">修复设计</v-btn>
        </v-col>
        <v-col align="center">
          <v-btn @click="btnToSpace('/space/exps/dataset_producer')">数据集收集</v-btn>
        </v-col>
        <v-col align="center">
          <v-btn>设置</v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-main>
</template>

<script>
export default {
  methods: {
    btnToSpace(url) {
      this.$router.push(url);
    },
  },
};
</script>