<!--  -->
<template>
  <div id="render_space"></div>
</template>

<script>
import * as THREE from "three";
import { OBJLoader } from 'three/addons/loaders/OBJLoader.js';
import { OrbitControls } from "three/addons/controls/OrbitControls.js";
import { TransformControls } from 'three/addons/controls/TransformControls.js';
import Stats from "stats-js";
import { invoke } from "@tauri-apps/api/tauri";
import bus from "vue3-eventbus";
import { color, instance } from "three/examples/jsm/nodes/Nodes.js";

export default {
  name: "ThreeScene",
  components: { THREE, OrbitControls, Stats },
  props: {},
  data() {
    //这里存放数据
    // !! https://stackoverflow.com/questions/65693108/threejs-component-working-in-vuejs-2-but-not-3/65732553
    return {};
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
      let width = window.innerWidth;
      let height = window.innerHeight * 0.95;

      this.camera = new THREE.PerspectiveCamera(75, width / height, 0.01, 1000);
      //创建渲染器
      this.renderer = new THREE.WebGLRenderer({ antialias: true });
      this.renderer.setSize(width, height);
      this.renderer.setClearColor(0xffffff, 1);
      this.renderer.setPixelRatio(window.devicePixelRatio);
      this.renderer.shadowMap.enabled = true;
      this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
      document
        .getElementById("render_space")
        .appendChild(this.renderer.domElement);

      // 创建坐标轴
      var axes = new THREE.AxesHelper(100);
      this.scene.add(axes);

      this.camera.position.x = 30;
      this.camera.position.y = 15;
      this.camera.position.z = 30;
      this.camera.lookAt(this.scene.position);
      this.scene.add(this.camera);

      const ambientLight = new THREE.AmbientLight(0x444444, 0.6);
      this.scene.add(ambientLight);
      const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
      directionalLight.position.set(1, 1, 1);
      this.scene.add(directionalLight);
      const directionalLight2 = new THREE.DirectionalLight(0xffffff, 0.5);
      directionalLight2.position.set(1, -1, -1);
      this.scene.add(directionalLight2);
      const directionalLight3 = new THREE.DirectionalLight(0xffffff, 0.3);
      directionalLight3.position.set(-1, -0.5, 0.0);
      this.scene.add(directionalLight3);

      // 控制器，监听鼠标事件
      this.controls = new OrbitControls(this.camera, this.renderer.domElement);
      this.controls.addEventListener("change", () => {
        this.renderer.render(this.scene, this.camera);
      });
      window.addEventListener(
        "resize",
        () => {
          this.camera.aspect = window.innerWidth / (window.innerHeight * 0.95);
          this.camera.updateProjectionMatrix();
          this.renderer.setSize(window.innerWidth, window.innerHeight * 0.95);
        },
        false
      );
      this.transformControls = new TransformControls(this.camera, this.renderer.domElement);
      this.scene.add(this.transformControls);
      this.transformControls.addEventListener("change", () => {
        this.render_scene()
      });
      this.transformControls.addEventListener("mouseDown", () => {
        this.controls.enabled = false;
      });
      this.transformControls.addEventListener("mouseUp", () => {
        this.controls.enabled = true;
      });
    },
    render_scene() {
      this.renderer.render(this.scene, this.camera);
    },
    animate() { },
    _load_OBJ(Obj, name) {
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

      this.render_scene();
    },
    add_mesh_to_scene(mesh, token) {
      mesh.name = token;
      this.scene.add(mesh);
      this.render_scene();
    },
    _change_mesh_colors(name, colormap) {
      let geometry = this.scene.getObjectByName(name).geometry;

      // 确保对象的geometry包含顶点信息
      if (geometry instanceof THREE.BufferGeometry) {
        var colorAttribute = geometry.getAttribute("color");

        if (
          colorAttribute !== undefined &&
          colorAttribute.count == colormap.length
        ) {
          for (var vind = 0; vind < colorAttribute.count; ++vind) {
            const _color = new THREE.Color(colormap[vind]);
            colorAttribute.setXYZ(vind, _color.r, _color.g, _color.b);
          }
          colorAttribute.needsUpdate = true;
        } else {
          console.error(
            "color change err: ",
            colorAttribute.count,
            colormap.length
          );
        }
      }
      this.render_scene();
    },
    _change_mesh_visibility(name, visible) {
      let mesh = this.scene.getObjectByName(name);
      mesh.material.visible = visible;
      this.render_scene();
    },
    _apply_mesh_transform(name, transform) {
      let mesh = this.scene.getObjectByName(name);
      console.log(mesh);
      mesh.applyMatrix4(new THREE.Matrix4(transform));
      mesh.updateMatrixWorld();
      console.log(mesh);
      // mesh.children[0].geometry.center();
      this.render_scene();
    },
    _set_transform_control(name, mode) {
      // mode in ['translate', 'rotate', 'scale']
      this.transformControls.setMode(mode);
      // this.transformControls.setSpace('local');
      const obj = this.scene.getObjectByName(name);
      this.transformControls.attach(obj);
      this.render_scene();
    },
    _detach_transform_control() {
      this.transformControls.detach();
      this.render_scene();
    }
  },
  //生命周期 - 创建完成（可以访问当前this实例）
  created() { },
  //生命周期 - 挂载完成（可以访问DOM元素）
  mounted() {
    this.init();
    this.render_scene();

    bus.on('meta-teeth/new-mesh-added', (param) => {
      this.add_mesh_to_scene(param.mesh, param.token);
    });
    bus.on("add-obj-to-scene", (param) => {
      this._load_OBJ(param.obj, param.token);
    });
    bus.on("change-vertex-color", (param) => {
      this._change_mesh_colors(param.name, param.colormap);
    });
    bus.on("meta-teeth/change-mesh-visibility", (param) => {
      this._change_mesh_visibility(param.name, param.visible);
    });
    bus.on("meta-teeth/apply-mesh-transform", (param) => {
      this._apply_mesh_transform(param.name, param.transform);
    });
    bus.on("set-mesh-transform-helper", (param) => {
      this._set_transform_control(param.name, param.mode);
    });
    bus.on("detach-mesh-transform-helper", (param) => {
      this._detach_transform_control();
    });
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