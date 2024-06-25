<template>
  <div ref="sceneContainer" class="scene-container"></div>
</template>

<script>
import * as THREE from "three";
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import bus from "vue3-eventbus";

const width = 382;
const height = 274;

export default {
  name: "FDIViewer",
  mounted() {
    this.initThree();
    this.render_scene();
    bus.on("set-preview-on", () => {
      const container = this.$refs.sceneContainer;
      container.style.display = "block";
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
      container.style.display = "none";
      // 创建场景
      // const width = container.clientWidth;
      // const height = container.clientHeight;

      // 创建相机
      this.camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
      this.camera.position.z = 8;
      this.camera.lookAt(this.scene.position);
      this.scene.add(this.camera);

      // 创建渲染器
      this.renderer = new THREE.WebGLRenderer();
      this.renderer.setSize(width, height);
      this.renderer.setClearColor(0xfffbfe, 1);
      this.renderer.setPixelRatio(window.devicePixelRatio);
      this.renderer.shadowMap.enabled = true;
      this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
      container.appendChild(this.renderer.domElement);

      const ambientLight = new THREE.AmbientLight(0x444444, 0.8);
      this.scene.add(ambientLight);
      const directionalLight = new THREE.DirectionalLight(0xffffff, 0.6);
      directionalLight.position.set(1, 1, 1);
      this.scene.add(directionalLight);
      const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.6);
      directionalLight2.position.set(1, -1, -1);
      this.scene.add(directionalLight2);
      const directionalLight3 = new THREE.DirectionalLight(0xffffff, 0.6);
      directionalLight2.position.set(-1, -1, 1);
      this.scene.add(directionalLight3);

      // 创建一个简单的立方体
      // const geometry = new THREE.BoxGeometry();
      // const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
      // const cube = new THREE.Mesh(geometry, material);
      // this.scene.add(cube);

      this.controls = new OrbitControls(this.camera, this.renderer.domElement);
      this.controls.addEventListener("change", () => {
        this.render_scene();
      });
    },
    render_scene() {
      this.renderer.render(this.scene, this.camera);
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
      mesh.receiveShadow = true;
      mesh.name = name;

      this.scene.add(mesh);
      this.camera.position.z = 10;
      this.camera.lookAt(this.scene.position);
    },
  },
};
</script>

<style scoped>
.scene-container {
  width: 382px;
  height: 274px;
}
</style>