<template>
  <div ref="sceneContainer" class="scene-container"></div>
</template>

<script>
import * as THREE from "three";

export default {
  name: "PreviewScene",
  mounted() {
    this.initThree();
  },
  methods: {
    initThree() {
      const container = this.$refs.sceneContainer;

      // 创建场景
      const scene = new THREE.Scene();

      // 创建相机
      const camera = new THREE.PerspectiveCamera(
        75,
        container.clientWidth / container.clientHeight,
        0.1,
        1000
      );
      camera.position.z = 5;

      // 创建渲染器
      const renderer = new THREE.WebGLRenderer();
      renderer.setSize(container.clientWidth, container.clientHeight);
      container.appendChild(renderer.domElement);

      // 创建一个简单的立方体
      const geometry = new THREE.BoxGeometry();
      const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
      const cube = new THREE.Mesh(geometry, material);
      scene.add(cube);

      // 渲染循环
      const animate = () => {
        requestAnimationFrame(animate);
        cube.rotation.x += 0.01;
        cube.rotation.y += 0.01;
        renderer.render(scene, camera);
      };

      animate();
    },
  },
};
</script>

<style scoped>
.scene-container {
  width: 100%;
  height: 400px;
}
</style>