<template>
    <v-text-field v-for="(hint, index) in rawInputHints" :key="index" density="compact" width="200" readonly
        prepend-inner-icon="mdi-upload" clearable @mousedown:control="clickAddRawInput(index)"
        @click:clear="clickRemoveRawInput(index)" v-model="rawInputs[index].name" variant="outlined" :label="hint"
        loading :disabled="rawInputs[index].loading">
        <template v-slot:loader>
            <v-progress-linear :active="rawInputs[index].loading" color="success" height="5" indeterminate />
        </template>
    </v-text-field>
</template>

<script>
import { open } from "@tauri-apps/api/dialog";
import { getFileNameFromPath, getHashToken } from "@/scripts/Utils";
import { APIRegister } from "@/scripts/APIs";
import { loadMeshUtil, exportPLY, registrationJaw } from "@/scripts/MeshTools";
import bus from "vue3-eventbus";

export default {
  name: "DPStep1",
  data: () => ({
    rawInputs: [
      { name: null, loading: false, token: "", bin: null, o3d: null },
      { name: null, loading: false, token: "", bin: null, o3d: null },
      { name: null, loading: false, token: "", bin: null, o3d: null },
      { name: null, loading: false, token: "", bin: null, o3d: null },
    ],
    rawInputHints: ["上颌 / Upper Jaw", "下颌 / Lower Jaw", "咬合左侧 / Bite Left", "咬合右侧 / Bite Right"],
    plyBinCache: [null, null, null, null],
    sceneMeshes: [],
  }),
  mounted() {},
  methods: {
    async clickAddRawInput(pos) {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "mesh", extensions: ["obj", "stl", "ply"] }],
      });
      if (filePath != null) {
        this.rawInputs[pos].loading = true;

        loadMeshUtil(
          filePath,
          object3D => {
            const bin = exportPLY(object3D);
            const localToken = getHashToken(bin);
            bus.emit('meta-teeth/new-mesh-added', { mesh: object3D, token: localToken });
            this.rawInputs[pos].name = getFileNameFromPath(filePath);
            this.rawInputs[pos].o3d = object3D;
            this.rawInputs[pos].bin = bin;
            this.rawInputs[pos].token = localToken;
            this.rawInputs[pos].loading = false;
          },
          () => { }, // process callback
          err => console.error('[ERROR] <loadMeshUtil>', err)
        );
      }
    },
    async forward(next) {
      let readyToNext = 0;
      this.sceneMeshes = [];
      for (let pos = 0; pos < this.rawInputs.length; ++pos) {
        if (!this.rawInputs[pos].token || !this.rawInputs[pos].bin)
          continue;

        this.rawInputs[pos].loading = true;
        readyToNext |= 1 << pos;

        // registration
        registrationJaw(this.rawInputs[pos].o3d);
        // 
        APIRegister(
          this.rawInputs[pos].bin,
          resp => {
            if (resp.status !== 200) {
              console.error('[ERROR] <APIRegister>', resp.status);
              return;
            }
            if (resp.data.token !== this.rawInputs[pos].token) {
              console.error('[ERROR] <APIRegister> token diff', resp.data.token, this.rawInputs[pos].token);
              return;
            }
            this.rawInputs[pos].loading = false;
            this.rawInputs[pos].o3d = null;
            this.rawInputs[pos].bin = null;
            this.sceneMeshes.push(this.rawInputs[pos].token);
            
            readyToNext &= ~(1 << pos);
            if (readyToNext === 0) {
              next();
            }
          },
          err => console.error('[ERROR] <APIRegister>', err)
        );
      }
    },
    async clickRemoveRawInput(pos) {
      this.rawInputs[pos] = { name: null, loading: false, token: "", bin: null, o3d: null };
    },
    canGoNext() {
      return !this.rawInputs.find(r => r.loading) && !!this.rawInputs.find((r, ind) => ind < 2 && r.token.length > 0);
    }
  },
};
</script>