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
import { loadMeshUtil, exportPLY, sampleMesh } from "@/scripts/MeshTools";
import bus from "vue3-eventbus";
import { invoke } from "@tauri-apps/api";
import { Object3D } from "three";

export default {
  name: "DPStep1",
  data: () => ({
    rawInputs: [
      { name: null, loading: false, token: "", bin: null },
      { name: null, loading: false, token: "", bin: null },
      { name: null, loading: false, token: "", bin: null },
      { name: null, loading: false, token: "", bin: null },
    ],
    rawInputHints: ["上颌 / Upper Jaw", "下颌 / Lower Jaw", "咬合左侧 / Bite Left", "咬合右侧 / Bite Right"],
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
            this.rawInputs[pos].bin = object3D;
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
      for (let pos = 0; pos < this.rawInputs.length; ++pos) {
        if (!this.rawInputs[pos].token || !this.rawInputs[pos].bin)
          continue;

        this.rawInputs[pos].loading = true;
        readyToNext |= 1 << pos;

        // registration
        if (pos <= 1) { // for jaws
          const obj = this.rawInputs[pos].bin;
          let [verts, norms] = sampleMesh(obj, 5000);
          invoke('backend_registration', { verts: verts, norms: norms, target: 'STANDARD-JAW' })
            .then((transform) => {
              console.log(transform);
              bus.emit("meta-teeth/apply-mesh-transform", { name: this.rawInputs[pos].token, transform });
            })
            .catch((err) => {
              console.error('[ERROR] <backend_registration>', err);
            });
        }
        APIRegister(
          exportPLY(this.rawInputs[pos].bin),
          resp => {
            if (resp.status !== 200) {
              console.error('[ERROR] <APIRegister>', resp.status);
              return;
            }
            if (resp.data.token !== this.rawInputs[pos].token) {
              resp.data.token = '';
              console.error('[ERROR] <APIRegister> token diff', resp.data.token, this.rawInputs[pos].token);
              return;
            }
            this.rawInputs[pos].loading = false;
            this.rawInputs[pos].bin = null;
            
            readyToNext &= ~(1 << pos);
            if (readyToNext === 0) {
              // hide-all
              this.setAllVisibleOrNot(false);
              next();
            }
          },
          err => console.error('[ERROR] <APIRegister>', err)
        );
      }
    },
    async clickRemoveRawInput(pos) {
      this.rawInputs[pos] = { name: null, loading: false, token: "", bin: null };
    },
    canGoNext() {
      return !this.rawInputs.find(r => r.loading) && !!this.rawInputs.find((r, ind) => ind < 2 && r.token.length > 0);
    },
    setAllVisibleOrNot(vis) {
      this.rawInputs.forEach(param => {
        if (!!param.token) {
          bus.emit('meta-teeth/change-mesh-visibility', { name: param.token, visible: vis });
        }
      });
    }
  },
};
</script>