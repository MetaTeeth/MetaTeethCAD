<!--  -->
<template>
  <div id="container"></div>
</template>

<script>
import * as THREE from "three";
import { OBJLoader } from "three/examples/jsm/loaders/OBJLoader";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { VertexNormalsHelper } from 'three/addons/helpers/VertexNormalsHelper.js';
import Stats from "stats-js";
import { invoke } from "@tauri-apps/api/tauri"
import { listen } from "@tauri-apps/api/event"

// async function backendLoadOBJ() {
//   return await 
// }

export default {
  //import引入的组件需要注入到对象中才能使用
  components: { THREE, OBJLoader, OrbitControls, Stats },
  props: {},
  data() {
    //这里存放数据
    // !! https://stackoverflow.com/questions/65693108/threejs-component-working-in-vuejs-2-but-not-3/65732553
    return {
    };
  },
  //监听属性 类似于data概念
  computed: {},
  //监控data中的数据变化
  watch: {},
  //方法集合
  methods: {
    init() {
      // 创建场景
      this.scene = new THREE.Scene();
      // 创建相机
      this.camera = new THREE.PerspectiveCamera(
        75,
        window.innerWidth / window.innerHeight,
        0.01,
        1000
      );
      //创建渲染器
      this.renderer = new THREE.WebGLRenderer({ antialias: true });
      this.renderer.setSize(window.innerWidth, window.innerHeight);
      this.renderer.setClearColor(0xffffff, 1);
      this.renderer.setPixelRatio(window.devicePixelRatio);
      this.renderer.shadowMap.enabled = true;
      // this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
      document.getElementById("container").appendChild(this.renderer.domElement);

      // 创建坐标轴
      var axes = new THREE.AxesHelper(100);
      this.scene.add(axes);

      this.camera.position.x = 18;
      this.camera.position.y = 6;
      this.camera.position.z = 17;
      this.camera.lookAt(this.scene.position);
      this.scene.add(this.camera);

      const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
      this.scene.add(ambientLight);
      const pointLight = new THREE.PointLight(0xffffff, 1.0)
      pointLight.position.set(30, 30, 30);
      pointLight.castShadow = true;
      this.scene.add(pointLight);

      // 控制器，监听鼠标事件
      this.controls = new OrbitControls(this.camera, this.renderer.domElement);
      this.controls.addEventListener("change", () => {
        this.renderer.render(this.scene, this.camera);
      });
      window.addEventListener("resize", () => {
        this.camera.aspect = window.innerWidth / window.innerHeight;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(window.innerWidth, window.innerHeight);
      }, false);

      listen('tauri://file-drop', event => {
        const filePath = event['payload'][0];
        if (filePath.endsWith('.obj')) {
          console.log(filePath);
          this.loadOBJ(filePath);
        }
      });
    },
    renderScene() {
      this.renderer.render(this.scene, this.camera);
    },
    animate() { },
    loadOBJ(filePath) {
      invoke('backend_load_obj', { filePath: filePath }).then((Obj) => {
        // flatten position and normal
        const positions = [];
        const normals = [];

        for (const vertex of Obj.vertices) {
          positions.push(...vertex.position);
          normals.push(...vertex.normal);
        }

        var geometry = new THREE.BufferGeometry();

        geometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(positions), 3));
        geometry.setAttribute('normal', new THREE.BufferAttribute(new Float32Array(normals), 3));

        geometry.setIndex(Obj.indices);

        const material = new THREE.MeshLambertMaterial({
          color: new THREE.Color("rgb(250, 250, 250)"),
          side: THREE.DoubleSide
        });

        const mesh = new THREE.Mesh(geometry, material);
        mesh.receiveShadow = true;

        const helper = new VertexNormalsHelper(mesh);
        this.scene.add(mesh);

        this.renderScene();
      });
    }
  },
  //生命周期 - 创建完成（可以访问当前this实例）
  created() {
  },
  //生命周期 - 挂载完成（可以访问DOM元素）
  mounted() {
    this.init();
    this.renderScene();
    // this.loadOBJ('static/tooth.obj');
  },
  beforeCreate() { }, //生命周期 - 创建之前
  beforeMount() { }, //生命周期 - 挂载之前
  beforeUpdate() { }, //生命周期 - 更新之前
  updated() { }, //生命周期 - 更新之后
  beforeDestroy() { }, //生命周期 - 销毁之前
  destroyed() { }, //生命周期 - 销毁完成
  activated() { }, //如果页面有keep-alive缓存功能，这个函数会触发
};
</script>
<style scoped></style>