<template>
  <v-dialog v-model="visible" max-width="875px" :update:modelValue="restartDialog()">
    <v-card>
      <v-card-title class="headline">缺损牙位选择器</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <!-- 第一部分：FDI 牙位选择器 -->
            <v-col class="d-flex" cols="12">
              <v-chip-group mandatory column>
                <div v-for="teeth in teethKinds" :key="teeth.order">
                  <v-chip :class="[{
                    picked: pickedTooth.some((item) => item.id === tooth.id),
                    selected: selectedTooth === tooth.id,
                  }, 'font-mono']" v-for="tooth in teeth.elements" :key="tooth.id" :text="tooth.name"
                    :value="tooth.name" rounded variant="outlined" @click="selectTooth(tooth.id)">
                  </v-chip>
                </div>
              </v-chip-group>
            </v-col>
          </v-row>
        </v-container>
        <v-container>
          <v-row>
            <!-- 第二部分和第三部分右半部分 -->
            <v-col class="d-flex" cols="6">
              <!-- 第二部分：FDIViewer 预览 -->
              <FDIViewer />
            </v-col>
            <v-col>
              <!-- 第三部分：详细配置表单 -->
              <v-form v-if="selectedTooth" :update:modelValue="pickTooth()">
                <v-select v-model="formData.sel_datasource" :items="toothOptions" label="牙位数据来源" required
                  variant="outlined" density="comfortable"></v-select>
                <v-checkbox v-model="formData.ckbx_cache" label="启用缓存"></v-checkbox>
                <v-checkbox v-model="formData.ckbx_oppo" label="参考对称牙齿形态"></v-checkbox>
                <v-switch v-if="switchValid" v-model="switchOn" color="success" label="标记为残缺牙齿" hide-details
                  density="comfortable"></v-switch>
                <v-switch v-else v-model="switchOn" disabled color="success" label="标记为残缺牙齿" hide-details
                  density="comfortable"></v-switch>
              </v-form>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn variant="plain" @click="visible = false" text="取消"></v-btn>
        <v-btn v-if="pickedTooth.length > 0" color="primary" variant="tonal" @click="saveSettings" text="下一步" />
        <v-btn v-else color="primary" variant="tonal" @click="saveSettings" text="下一步" disabled />
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script>
import FDIViewer from "../FDIViewer.vue"; // 假设你已经创建了这个组件
import { invoke } from "@tauri-apps/api/tauri";
import bus from "vue3-eventbus";

export default {
  name: "FormStep3",
  components: {
    FDIViewer,
  },
  data() {
    return {
      visible: false,
      switchOn: false,
      selectedTooth: null,
      pickedTooth: [], // [{ id, params }]
      teethValid: new Set(),
      teethKinds: [
        {
          order: 1,
          elements: [
            { id: 18, name: "18" },
            { id: 17, name: "17" },
            { id: 16, name: "16" },
            { id: 15, name: "15" },
            { id: 14, name: "14" },
            { id: 13, name: "13" },
            { id: 12, name: "12" },
            { id: 11, name: "11" },
          ],
        },
        {
          order: 2,
          elements: [
            { id: 21, name: "21" },
            { id: 22, name: "22" },
            { id: 23, name: "23" },
            { id: 24, name: "24" },
            { id: 25, name: "25" },
            { id: 26, name: "26" },
            { id: 27, name: "27" },
            { id: 28, name: "28" },
          ],
        },
        {
          order: 3,
          elements: [
            { id: 48, name: "48" },
            { id: 47, name: "47" },
            { id: 46, name: "46" },
            { id: 45, name: "45" },
            { id: 44, name: "44" },
            { id: 43, name: "43" },
            { id: 42, name: "42" },
            { id: 41, name: "41" },
          ],
        },
        {
          order: 4,
          elements: [
            { id: 31, name: "31" },
            { id: 32, name: "32" },
            { id: 33, name: "33" },
            { id: 34, name: "34" },
            { id: 35, name: "35" },
            { id: 36, name: "36" },
            { id: 37, name: "37" },
            { id: 38, name: "38" },
          ],
        },
      ],
      toothOptions: ["自动分割", "手动分割", "手动上传"],
      formData: {
        sel_datasource: "自动分割",
        ckbx_cache: false,
        ckbx_oppo: false,
      },
      switchValid: false,
    };
  },
  mounted() {
    bus.on("set-tooth-segment-ready", (param) => {
      this.teethValid.add(param.label);
    });
  },
  methods: {
    restartDialog() {
      if (!this.visible) {
        bus.emit("set-preview-visible", { visible: false });
      }
      if (this.visible && this.selectedTooth) {
        bus.emit("set-preview-visible", { visible: true });
      }
    },
    selectTooth(toothId) {
      if (!this.selectTooth) {
        // first pick
        bus.emit("set-preview-visible", { visible: true });
      }

      this.selectedTooth = toothId;
      this.switchValid = this.teethValid.has(`${toothId}`);
      const ind = this.pickedTooth.findIndex((item) => item.id === toothId);

      if (ind == -1) {
        // default
        // 重置表单数据
        if (this.switchValid) {
          this.toothOptions = ["自动分割", "手动分割", "手动上传"];
        } else {
          this.toothOptions = ["手动分割", "手动上传"];
        }
        this.switchOn = false;
        this.formData.sel_datasource = this.toothOptions[0];
        this.formData.ckbx_cache = false;
        this.formData.ckbx_oppo = false;
      } else {
        // load config
        this.switchOn = true;
        this.formData.sel_datasource = this.pickedTooth[ind].sel_datasource;
        this.formData.ckbx_cache = this.pickedTooth[ind].ckbx_cache;
        this.formData.ckbx_oppo = this.pickedTooth[ind].ckbx_oppo;
      }

      if (this.switchValid) {
        // set preview
        invoke("backend_getmesh", { token: `autoseg_${toothId}` }).then(
          (obj) => {
            bus.emit("set-obj-to-preview", {
              obj: obj,
              name: `autoseg_${toothId}`,
            });
          }
        );
      } else {
        bus.emit("clear-preview", {});
      }
    },
    pickTooth() {
      const ind = this.pickedTooth.findIndex(
        (item) => item.id === this.selectedTooth
      );
      if (ind == -1 && this.switchOn) {
        // append config to pickedTooth
        this.pickedTooth.push({
          id: this.selectedTooth,
          sel_datasource: this.formData.sel_datasource,
          ckbx_cache: this.formData.ckbx_cache,
          ckbx_oppo: this.formData.ckbx_oppo,
        });
      } else if (ind != -1 && !this.switchOn) {
        this.pickedTooth.splice(ind, 1);
      } else if (ind != -1 && this.switchOn) {
        // modify
        this.pickedTooth[ind].sel_datasource = this.formData.sel_datasource;
        this.pickedTooth[ind].ckbx_cache = this.formData.ckbx_cache;
        this.pickedTooth[ind].ckbx_oppo = this.formData.ckbx_oppo;
      }
    },
    saveSettings() {
      this.visible = false;
      bus.emit("go-to-step", { step: 4 });
    },
  },
};
</script>

<style scoped>
.selected {
  background-color: #42a5f5;
}

.picked {
  background-color: #60e774;
}

.font-mono {
  font-family:  monospace;
}
</style>