<template>
  <v-dialog v-model="visible" max-width="700">
    <v-card prepend-icon="mdi-account" title="牙列三维模型输入">
      <v-img :aspect-ratio="16 / 6" src="https://imgbed.scubot.com/image/202406011621069.png" cover>
      </v-img>
      <v-card-text>
        <v-row dense>
          <v-col cols="6" >
            <v-file-input label="上颌 / Upper Jaw" accept=".obj" show-size ></v-file-input>
          </v-col>
          <v-col cols="6" >
            <v-file-input label="下颌 / Lower Jaw" accept=".obj" show-size ></v-file-input>
          </v-col>
        </v-row>
        <v-row dense>
          <v-col cols="6" >
            <v-file-input label="咬合 / Bite Scan" accept=".obj" show-size ></v-file-input>
          </v-col>
          <v-col cols="6" >
            <v-file-input label="咬合（异侧） / Bite Scan" accept=".obj" show-size ></v-file-input>
          </v-col>
        </v-row>

        <small class="text-caption text-medium-emphasis"><strong>目前仅支持OBJ文件</strong></small>
      </v-card-text>

      <v-divider></v-divider>

      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn text="取消" variant="plain" @click="visible = false"></v-btn>

        <v-btn color="primary" text="确认" variant="tonal" @click="visible = false"></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
export default {
  name: "FormStep1",
  data: () => ({
    visible: false,
  }),
}
</script>