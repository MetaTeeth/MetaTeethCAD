<template>
  <div ref="sceneContainer" class="scene-container" v-show="isVisible"></div>
</template>

<script>
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import { EffectComposer } from 'three/addons/postprocessing/EffectComposer.js';
import { RenderPass } from 'three/addons/postprocessing/RenderPass.js';
import { OutlinePass } from 'three/addons/postprocessing/OutlinePass.js';
import bus from "vue3-eventbus";

const width = 382;
const height = 274;

export default {
  name: "FDIViewer",
  data() {
    return {
      isVisible: false,
    };
  },
  mounted() {
    this.initThree();
    bus.on("set-preview-visible", (param) => {
      this.isVisible = param.visible;
      if (this.isVisible) {
        this.render_scene();
      }
    });
    bus.on("set-obj-to-preview", (param) => {
      this._set_OBJ(param.obj, param.name);
      this.render_scene();
    });
    bus.on("clear-preview", () => {
      this._clear_OBJ();
      this.render_scene();
    });
  },
  methods: {
    initThree() {
      this.scene = new THREE.Scene();

      const container = this.$refs.sceneContainer;
      // 创建场景
      // const width = container.clientWidth;
      // const height = container.clientHeight;

      // 创建相机
      this.camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
      this.camera.position.z = 9;
      this.camera.lookAt(new THREE.Vector3(0, 0, 0));
      this.scene.add(this.camera);

      // 创建渲染器
      const renderer = new THREE.WebGLRenderer();
      renderer.setSize(width, height);
      renderer.setClearColor(0xfffbfe, 1);
      renderer.setPixelRatio(window.devicePixelRatio);
      renderer.shadowMap.enabled = true;
      renderer.shadowMap.type = THREE.PCFSoftShadowMap;

      this.composer = new EffectComposer(renderer);
      this.composer.setSize(width, height);
      container.appendChild(renderer.domElement);

      const ambientLight = new THREE.AmbientLight(0xffffff, 0.8);
      this.scene.add(ambientLight);
      const directionalLight = new THREE.DirectionalLight(0xffffff, 0.6);
      directionalLight.position.set(1, 1, 1);
      this.scene.add(directionalLight);
      const directionalLight3 = new THREE.DirectionalLight(0xffffff, 0.6);
      directionalLight3.position.set(-1, -1, 1);
      this.scene.add(directionalLight3);

      this.controls = new OrbitControls(this.camera, renderer.domElement);

      this.controls.addEventListener("change", () => {
        this.render_scene();
      });

      const renderPass = new RenderPass(this.scene, this.camera);
      this.composer.addPass(renderPass);

      const v2 = new THREE.Vector2(width, height);
      this.outlinePass = new OutlinePass(v2, this.scene, this.camera);
      this.outlinePass.edgeStrength = 3 //粗
      this.composer.addPass(this.outlinePass);

      this.render_scene();
    },
    render_scene() {
      // requestAnimationFrame(this.render_scene);

      // this.controls.update();
      this.composer.render();
    },
    _clear_OBJ() {
      this.scene.traverse((object) => {
        if (object instanceof THREE.Mesh) {
          object.geometry.dispose();
          this.scene.remove(object);
        }
      });
    },
    _set_OBJ(Obj, name) {
      this._clear_OBJ();
      // free all obj
      const positions = [];
      const colors = [];

      for (const vertex of Obj.vertices) {
        positions.push(...vertex.position);
        colors.push(...[230 / 255, 230 / 255, 230 / 255]);
      }

      var geometry = new THREE.BufferGeometry();

      geometry.verticesNeedUpdate = true;
      geometry.dynamic = true;
      geometry.setAttribute(
        "position",
        new THREE.BufferAttribute(new Float32Array(positions), 3)
      );
      geometry.setAttribute(
        "color",
        new THREE.BufferAttribute(new Float32Array(colors), 3)
      );

      geometry.setIndex(Obj.indices);
      geometry.computeVertexNormals();
      geometry.center();

      const material = new THREE.MeshPhongMaterial({
        // color: new THREE.Color("rgb(230, 230, 230)"),
        side: THREE.DoubleSide,
        vertexColors: true,
      });

      const mesh = new THREE.Mesh(geometry, material);
      // this.outlinePass.selectedObjects = [mesh];
      
      mesh.receiveShadow = true;
      mesh.name = name;

      this.scene.add(mesh);
      this.camera.position.z = 9;
      this.camera.lookAt(new THREE.Vector3(0, 0, 0));
    },
  },
  beforeDestroy() {
    this.controls.dispose();
    this.composer.dispose();
  },
};
</script>

<style scoped>
.scene-container {
  width: 382px;
  height: 274px;
}

.hidden {
  display: none;
}
</style>