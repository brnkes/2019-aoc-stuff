import * as THREE from 'three';
import GPUComputationRenderer from "./GPUComputationRenderer";

const spaceDims = {w: 5, h: 5};

const renderer = (() => {
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('webgl2', {alpha: false});

    return new THREE.WebGLRenderer(
        {
            antialias: false,
            context
        }
    );
})();

renderer.setSize(spaceDims.w, spaceDims.h);

const compute = new GPUComputationRenderer(spaceDims.w, spaceDims.h, renderer);

compute.createTexture();

