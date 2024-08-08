<template>
  <div v-for="(hint, index) in rawInputHints" :key="index">
    <v-row v-if="currentActivePos >= 0 && index < 2 && !!rawInputs[index].name" align="start" justify="center" dense no-gutters
      style="height: 55px;">
      <v-col cols="10">
        <v-text-field density="compact" width="180" readonly
          prepend-inner-icon="mdi-upload"
          @mousedown:control="clickAddRawInput(index)" @click:clear="clickRemoveRawInput(index)"
          v-model="rawInputs[index].name" variant="outlined" :label="hint" loading :disabled="rawInputs[index].loading || currentActivePos >= 0">
          <template v-slot:loader>
            <v-progress-linear :active="rawInputs[index].loading" color="success" height="5" indeterminate />
          </template>
        </v-text-field>
      </v-col>
      <v-col cols="2" align-self="start">
        <v-chip v-if="rawInputs[index].locked" color="success" variant="text" link style="left:8px;top:2px;height:36px;width:36px;" v-tooltip="'已锁定配准'">
          <v-icon color="success" icon="mdi-lock-check" size="x-large" start></v-icon>
        </v-chip>
        <v-chip v-else variant="text" color="warning" link style="left:8px;top:2px;height:36px;width:36px;" v-tooltip="'点击锁定配准'" @click="goNextActive()">
          <v-icon color="warning" icon="mdi-lock-open-remove-outline" size="x-large" start></v-icon>
        </v-chip>
      </v-col>
    </v-row>

    <v-row v-else align="start" justify="center" dense no-gutters style="height: 55px;">
      <v-col>
        <v-text-field density="compact" width="220" readonly prepend-inner-icon="mdi-upload"
          :clearable="currentActivePos >= 0"
          @mousedown:control="clickAddRawInput(index)" @click:clear="clickRemoveRawInput(index)"
          v-model="rawInputs[index].name" variant="outlined" :label="hint" loading :disabled="rawInputs[index].loading || currentActivePos >= 0">
          <template v-slot:loader>
            <v-progress-linear :active="rawInputs[index].loading" color="success" height="5" indeterminate />
          </template>
        </v-text-field>
      </v-col>
    </v-row>
  </div>
</template>

<script>
import { open } from "@tauri-apps/api/dialog";
import { getFileNameFromPath, getHashToken } from "@/scripts/Utils";
import { APIRegister } from "@/scripts/APIs";
import { loadMeshUtil, exportPLY, sampleMesh, shiftCentroid } from "@/scripts/MeshTools";
import bus from "vue3-eventbus";
import { invoke } from "@tauri-apps/api";
import { Object3D } from "three";
import { STANDARD_JAW } from "@/scripts/MeshStore";

export default {
  name: "DPStep1",
  data: () => ({
    rawInputs: [
      { name: null, loading: false, token: "", bin: null, locked: false },
      { name: null, loading: false, token: "", bin: null, locked: false },
      { name: null, loading: false, token: "", bin: null, locked: false },
      { name: null, loading: false, token: "", bin: null, locked: false },
    ],
    rawInputHints: ["上颌 / Upper Jaw", "下颌 / Lower Jaw", "咬合左侧 / Bite Left", "咬合右侧 / Bite Right"],
    nextFunc: () => { },
    currentActivePos: -1,
  }),
  mounted() { },
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
            bus.emit('meta-teeth/mesh-added', { mesh: object3D, token: localToken });
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
      bus.emit('meta-teeth/mesh-added', {
        mesh: STANDARD_JAW,
        token: 'STANDARD-JAW',
        params: {
          color: 'rgb(230, 100, 100)',
          opacity: 0.7
        }
      });

      this.setAllVisibleOrNot(false);
      this.nextFunc = next;
      this.goNextActive();

      // let readyToNext = 0;
      // for (let pos = 0; pos < this.rawInputs.length; ++pos) {
      //   if (!this.rawInputs[pos].token || !this.rawInputs[pos].bin)
      //     continue;

      //   this.rawInputs[pos].loading = true;
      //   readyToNext |= 1 << pos;

      //   // registration
      //   if (pos <= 1) { // for jaws
      //     // mask
      //     const maskName = `PRELOAD-STANDARD-JAW-${pos}`;
      //     bus.emit('meta-teeth/mesh-added', {
      //       mesh: STANDARD_JAW,
      //       token: maskName,
      //       params: {
      //         color: 'rgb(230, 100, 100)',
      //         opacity: 0.7
      //       }
      //     });

      //     let obj = this.rawInputs[pos].bin;
      //     obj = shiftCentroid(obj, STANDARD_JAW);
      //     bus.emit('meta-teeth/set-mesh-transform-helper', { name: this.rawInputs[pos].token, mode: 'rotate' });
          // let [verts, norms] = sampleMesh(obj, 50000);
          // invoke('backend_registration', { verts: verts, norms: norms, target: 'STANDARD-JAW' })
          //   .then((transform) => {
          //     bus.emit("meta-teeth/apply-mesh-transform", { name: this.rawInputs[pos].token, transform: transform });
          //   })
          //   .catch((err) => {
          //     console.error('[ERROR] <backend_registration>', err);
          //   });
        // }
        // APIRegister(
        //   exportPLY(this.rawInputs[pos].bin),
        //   resp => {
        //     if (resp.status !== 200) {
        //       console.error('[ERROR] <APIRegister>', resp.status);
        //       return;
        //     }
        //     if (resp.data.token !== this.rawInputs[pos].token) {
        //       resp.data.token = '';
        //       console.error('[ERROR] <APIRegister> token diff', resp.data.token, this.rawInputs[pos].token);
        //       return;
        //     }
        //     this.rawInputs[pos].loading = false;
        //     this.rawInputs[pos].bin = null;

        //     readyToNext &= ~(1 << pos);
        //     if (readyToNext === 0) {
        //       // hide-all
        //       this.setAllVisibleOrNot(false);
        //       next();
        //     }
        //   },
        //   err => console.error('[ERROR] <APIRegister>', err)
        // );
      // }
    },

    async clickRemoveRawInput(pos) {
      this.rawInputs[pos] = { name: null, loading: false, token: "", bin: null, locked: false };
    },
    canGoNext() {
      return !this.rawInputs.find(r => r.loading)
        && !!this.rawInputs.find((r, ind) => ind < 2 && r.token.length > 0)
        && this.currentActivePos < 0;
    },
    setAllVisibleOrNot(vis) {
      this.rawInputs.forEach(param => {
        if (!!param.token) {
          bus.emit('meta-teeth/change-mesh-visibility', { name: param.token, visible: vis });
        }
      });
    },
    goNextActive() {
      // after press 'next' button or locked
      if (this.currentActivePos >= 0) {
        bus.emit('meta-teeth/change-mesh-visibility', { name: this.rawInputs[this.currentActivePos].token, visible: false });
        // upload
        this.rawInputs[this.currentActivePos].locked = true;
        this.uploadMesh(this.currentActivePos);
      }
      // go next
      this.currentActivePos += 1;
      while (this.currentActivePos < 4 && !this.rawInputs[this.currentActivePos].name) {
        this.currentActivePos += 1;
      }
      if (this.currentActivePos > 1) { // all handled
        while (this.currentActivePos < 4) {
          if (!this.rawInputs[this.currentActivePos].name) continue;
          this.uploadMesh(this.currentActivePos);
          this.currentActivePos += 1;
        }
        bus.emit('meta-teeth/detach-mesh-transform-helper', {});
        bus.emit('meta-teeth/change-mesh-visibility', { name: 'STANDARD-JAW', visible: false });
        this.currentActivePos = -1;
        this.nextFunc();
      }
      else {
        // set transform handle
        bus.emit('meta-teeth/change-mesh-visibility', { name: this.rawInputs[this.currentActivePos].token, visible: true });
        const translate = shiftCentroid(this.rawInputs[this.currentActivePos].bin, STANDARD_JAW);
        bus.emit('meta-teeth/apply-mesh-translate', { name: this.rawInputs[this.currentActivePos].token, translate: translate });
        bus.emit('meta-teeth/set-mesh-transform-helper', { name: this.rawInputs[this.currentActivePos].token, mode: 'rotate' });
      }
    },
    uploadMesh(pos) {
      this.rawInputs[pos].loading = true;

      // get final obj
      console.log('---- UPLOAD ----');
      // upload

      this.rawInputs[pos].loading = false;
    }
  },
};
</script>