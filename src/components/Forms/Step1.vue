<template>
  <v-dialog v-model="visible" max-width="700">
    <v-card prepend-icon="mdi-account" title="牙列三维模型输入">
      <v-img
        :aspect-ratio="16 / 6"
        src="https://imgbed.scubot.com/image/202406011621069.png"
        cover
      >
      </v-img>
      <v-card-text>
        <v-row dense>
          <v-col cols="6">
            <!-- <v-file-input variant="outlined" label="上颌 / Upper Jaw" accept="model/obj"></v-file-input> -->
            <v-text-field
              prepend-inner-icon="mdi-upload"
              readonly
              @mousedown:control="clickUploadOBJ(0)"
              v-model="names[0]"
              variant="outlined"
              label="上颌 / Upper Jaw"
              loading
            >
              <template v-slot:loader>
                <v-progress-linear
                  :active="flags_load[0]"
                  color="success"
                  height="5"
                  indeterminate
                ></v-progress-linear>
              </template>
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <!-- <v-file-input variant="outlined" label="下颌 / Lower Jaw" accept="model/obj"></v-file-input> -->
            <v-text-field
              prepend-inner-icon="mdi-upload"
              readonly
              @mousedown:control="clickUploadOBJ(1)"
              v-model="names[1]"
              variant="outlined"
              label="下颌 / Lower Jaw"
              loading
            >
              <template v-slot:loader>
                <v-progress-linear
                  :active="flags_load[1]"
                  color="success"
                  height="5"
                  indeterminate
                ></v-progress-linear>
              </template>
            </v-text-field>
          </v-col>
        </v-row>
        <v-row dense>
          <v-col cols="6">
            <!-- <v-file-input variant="outlined" label="咬合 / Bite Scan" accept="model/obj"></v-file-input> -->
            <v-text-field
              prepend-inner-icon="mdi-upload"
              readonly
              @mousedown:control="clickUploadOBJ(2)"
              v-model="names[2]"
              variant="outlined"
              label="咬合 / Bite Scan"
              loading
            >
              <template v-slot:loader>
                <v-progress-linear
                  :active="flags_load[2]"
                  color="success"
                  height="5"
                  indeterminate
                ></v-progress-linear>
              </template>
            </v-text-field>
          </v-col>
          <v-col cols="6">
            <!-- <v-file-input variant="outlined" label="咬合（异侧） / Bite Scan" accept="model/obj"></v-file-input> -->
            <v-text-field
              prepend-inner-icon="mdi-upload"
              readonly
              @mousedown:control="clickUploadOBJ(3)"
              v-model="names[3]"
              variant="outlined"
              label="咬合（异侧） / Bite Scan"
              loading
            >
              <template v-slot:loader>
                <v-progress-linear
                  :active="flags_load[3]"
                  color="success"
                  height="5"
                  indeterminate
                ></v-progress-linear>
              </template>
            </v-text-field>
          </v-col>
        </v-row>

        <small class="text-caption text-medium-emphasis"
          ><strong>目前仅支持OBJ文件</strong></small
        >
      </v-card-text>

      <v-divider></v-divider>

      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn text="取消" variant="plain" @click="visible = false"></v-btn>

        <v-btn
          v-if="
            flags_load[0] || flags_load[1] || flags_load[2] || flags_load[3]
          "
          color="primary"
          text="确认"
          variant="tonal"
          disabled
        ></v-btn>
        <v-btn
          v-else
          color="primary"
          text="确认"
          variant="tonal"
          @click="confirm()"
        ></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import { open, save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import bus from "vue3-eventbus";

function getFileNameFromPath(filePath) {
  const pathSeparator = filePath.includes("/") ? "/" : "\\";
  const fileName = filePath.split(pathSeparator).pop();
  return fileName;
}

export default {
  name: "FormStep1",
  data: () => ({
    visible: false,
    files: [null, null, null, null],
    names: [null, null, null, null],
    flags_load: [false, false, false, false],
    objs: [null, null, null, null],
    tokens: ["", "", "", ""],
  }),
  methods: {
    async clickUploadOBJ(pos) {
      this.files[pos] = await open({
        multiple: false,
        filters: [
          {
            name: "OBJ",
            extensions: ["obj"],
          },
        ],
      });
      if (this.files[pos] != null) {
        this.names[pos] = getFileNameFromPath(this.files[pos]);
        this.flags_load[pos] = true;

        invoke("backend_load_obj", { filePath: this.files[pos] }).then(
          (OBJPack) => {
            this.objs[pos] = OBJPack.obj;
            this.tokens[pos] = OBJPack.token;
            this.flags_load[pos] = false;
          }
        );
      }
    },
    confirm() {
      for (var ind = 0; ind < 4; ++ind) {
        if (this.objs[ind] != null && this.tokens[ind].length > 10) {
          bus.emit("add-obj-to-scene", {
            obj: this.objs[ind],
            name: this.tokens[ind],
          });
        }
      }
      this.visible = false;

      bus.emit("go-to-step", { step: 2 });
    },
  },
};
</script>