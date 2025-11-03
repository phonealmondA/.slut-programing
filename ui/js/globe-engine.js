// Globe Engine for .slut Code Visualization
// Adapted from globe-editor-v2.js for Quantum Consciousness IDE

// Three.js scene components
let scene, camera, renderer, globeSphere;
let codeNodes = [];
let isRotating = false;
let labelsVisible = true;

// Camera controls
let isDragging = false;
let previousMousePosition = { x: 0, y: 0 };
let selectedNode = null;

// Raycaster for node selection
const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();

// Color mapping for .slut block types
const BLOCK_COLORS = {
    'Input': 0x00bcd4,            // Cyan - for firstInput, secondInput
    'Target': 0xff5722,           // Deep Orange - for targetNum
    'Result': 0xffc107,           // Yellow/Gold - for result operations
    'Variable': 0x2196f3,         // Blue - other variables
    'QuantumOperation': 0x9c27b0, // Purple
    'Loop': 0xf44336,             // Red
    'Condition': 0xe91e63,        // Pink
    'Function': 0xff9800,         // Orange
    'Output': 0x4caf50,           // Green
    'Single': 0x9e9e9e,           // Gray
    'Unknown': 0x757575           // Dark Gray
};

// CodeNode class to represent a code block in 3D space
class CodeNode {
    constructor(block, position, index) {
        this.block = block;          // The code block data
        this.position = position;    // 3D position on sphere
        this.index = index;          // Index in code array
        this.isActive = true;
        this.isExpanded = false;
        this.mesh = null;            // The 3D sphere mesh
        this.textSprite = null;      // Label sprite
        this.connections = [];       // Lines to other nodes
    }
}

// Initialize the globe visualization
function initGlobe() {
    console.log('Initializing globe visualization...');

    const container = document.getElementById('globe-canvas').parentElement;
    const width = container.clientWidth;
    const height = container.clientHeight;

    // Create scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1a1a2e);

    // Create camera
    camera = new THREE.PerspectiveCamera(75, width / height, 0.1, 1000);
    camera.position.set(0, 0, 8);
    camera.lookAt(0, 0, 0);

    // Create renderer
    renderer = new THREE.WebGLRenderer({
        canvas: document.getElementById('globe-canvas'),
        antialias: true,
        alpha: true
    });
    renderer.setSize(width, height);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;

    // Add brighter lights
    const ambientLight = new THREE.AmbientLight(0x808080, 1.2);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 1.5);
    directionalLight.position.set(5, 5, 5);
    directionalLight.castShadow = true;
    scene.add(directionalLight);

    const pointLight1 = new THREE.PointLight(0x667eea, 1.2);
    pointLight1.position.set(-5, 3, 5);
    scene.add(pointLight1);

    const pointLight2 = new THREE.PointLight(0x764ba2, 0.8);
    pointLight2.position.set(5, -3, -5);
    scene.add(pointLight2);

    // Set up controls
    setupMouseControls();
    setupWindowResize();

    // Start animation loop
    animate();

    console.log('Globe initialized successfully');
}

// Create the main sphere based on code blocks
function createGlobeSphere(blocks) {
    console.log('Creating globe sphere with', blocks.length, 'blocks');

    // Clear existing sphere and nodes
    clearGlobe();

    if (!blocks || blocks.length === 0) {
        console.log('No blocks to visualize');
        return;
    }

    // Calculate sphere size based on code complexity
    const sphereRadius = Math.max(3, Math.min(6, blocks.length * 0.15));

    // Create main sphere with brighter materials
    const sphereGeometry = new THREE.SphereGeometry(sphereRadius, 64, 32);
    const sphereMaterial = new THREE.MeshPhongMaterial({
        color: 0x667eea,
        transparent: true,
        opacity: 0.3,
        wireframe: false,
        emissive: 0x667eea,
        emissiveIntensity: 0.2,
        shininess: 80
    });

    globeSphere = new THREE.Mesh(sphereGeometry, sphereMaterial);
    globeSphere.castShadow = true;
    globeSphere.receiveShadow = true;
    scene.add(globeSphere);

    // Add brighter wireframe overlay
    const wireframeGeometry = new THREE.SphereGeometry(sphereRadius + 0.01, 32, 16);
    const wireframeMaterial = new THREE.MeshBasicMaterial({
        color: 0x888888,
        wireframe: true,
        transparent: true,
        opacity: 0.4
    });
    const wireframe = new THREE.Mesh(wireframeGeometry, wireframeMaterial);
    globeSphere.add(wireframe);

    // Create nodes on sphere surface
    createNodesOnSphere(blocks, sphereRadius);

    console.log('Globe sphere created with', codeNodes.length, 'nodes');
}

// Distribute nodes evenly on sphere using golden spiral
function createNodesOnSphere(blocks, radius) {
    codeNodes = [];

    const goldenAngle = Math.PI * (3 - Math.sqrt(5));

    blocks.forEach((block, index) => {
        // Calculate position using golden spiral
        const y = 1 - (index / (blocks.length - 1 || 1)) * 2;
        const radiusAtY = Math.sqrt(1 - y * y);
        const theta = goldenAngle * index;

        const x = Math.cos(theta) * radiusAtY * radius;
        const z = Math.sin(theta) * radiusAtY * radius;
        const yPos = y * radius;

        const position = new THREE.Vector3(x, yPos, z);
        const node = new CodeNode(block, position, index);

        // Create 3D mesh for node
        createNodeMesh(node, radius);

        codeNodes.push(node);
    });

    // Create connection lines after all nodes are created
    createConnectionLines(radius);
}

// Create curved connection lines between related nodes
function createConnectionLines(radius) {
    codeNodes.forEach(node => {
        if (!node.block.connections || node.block.connections.length === 0) return;

        node.block.connections.forEach(targetIndex => {
            const targetNode = codeNodes[targetIndex];
            if (!targetNode) return;

            // Get colors for gradient line
            const sourceColor = BLOCK_COLORS[node.block.type] || BLOCK_COLORS['Unknown'];
            const targetColor = BLOCK_COLORS[targetNode.block.type] || BLOCK_COLORS['Unknown'];

            // Create curved line along sphere surface
            const curve = createSphereArcCurve(
                node.position,
                targetNode.position,
                radius
            );

            const points = curve.getPoints(50);
            const geometry = new THREE.BufferGeometry().setFromPoints(points);

            // Create gradient material
            const material = new THREE.LineBasicMaterial({
                color: sourceColor,
                transparent: true,
                opacity: 0.6,
                linewidth: 2
            });

            const line = new THREE.Line(geometry, material);
            scene.add(line);
            node.connections.push(line);
        });
    });
}

// Create a curved path along the sphere surface between two points
function createSphereArcCurve(start, end, radius) {
    const points = [];
    const steps = 50;

    // Get normalized directions
    const startNorm = start.clone().normalize();
    const endNorm = end.clone().normalize();

    // Calculate angle between vectors
    const angle = startNorm.angleTo(endNorm);

    // Create rotation axis
    const axis = new THREE.Vector3().crossVectors(startNorm, endNorm).normalize();

    // If vectors are parallel, choose arbitrary perpendicular axis
    if (axis.length() < 0.001) {
        axis.set(1, 0, 0);
        if (Math.abs(startNorm.x) > 0.9) {
            axis.set(0, 1, 0);
        }
    }

    // Generate points along the arc
    for (let i = 0; i <= steps; i++) {
        const t = i / steps;
        const currentAngle = angle * t;

        // Rotate start vector around axis
        const point = startNorm.clone();
        point.applyAxisAngle(axis, currentAngle);
        point.multiplyScalar(radius + 0.02); // Slightly above sphere surface

        points.push(point);
    }

    return new THREE.CatmullRomCurve3(points);
}

// Create the visual mesh for a code node
function createNodeMesh(node, sphereRadius) {
    const nodeGeometry = new THREE.SphereGeometry(0.15, 16, 16);

    // Get color based on block type
    const blockType = node.block.type || 'Unknown';
    const color = BLOCK_COLORS[blockType] || BLOCK_COLORS['Unknown'];

    const nodeMaterial = new THREE.MeshPhongMaterial({
        color: color,
        shininess: 100,
        emissive: color,
        emissiveIntensity: 0.4
    });

    node.mesh = new THREE.Mesh(nodeGeometry, nodeMaterial);
    node.mesh.position.copy(node.position);
    node.mesh.castShadow = true;
    node.mesh.receiveShadow = true;
    node.mesh.userData.nodeIndex = node.index;

    scene.add(node.mesh);

    // Create label if needed
    if (labelsVisible) {
        createNodeLabel(node, sphereRadius);
    }
}

// Create a text label for a node
function createNodeLabel(node, sphereRadius) {
    const canvas = document.createElement('canvas');
    const context = canvas.getContext('2d');

    // Get the actual code line to display
    let labelText = node.block.lines && node.block.lines[0] ? node.block.lines[0] : 'Block';

    // Truncate if too long
    if (labelText.length > 25) {
        labelText = labelText.substring(0, 22) + '...';
    }

    // Measure text to size canvas appropriately
    context.font = 'bold 11px Courier New';
    const textWidth = context.measureText(labelText).width;
    canvas.width = Math.max(100, textWidth + 20);
    canvas.height = 32;

    // Draw background with gradient
    const gradient = context.createLinearGradient(0, 0, 0, canvas.height);
    gradient.addColorStop(0, 'rgba(102, 126, 234, 0.95)');
    gradient.addColorStop(1, 'rgba(118, 75, 162, 0.95)');
    context.fillStyle = gradient;
    context.fillRect(0, 0, canvas.width, canvas.height);

    // Draw border
    context.strokeStyle = '#ffffff';
    context.lineWidth = 2;
    context.strokeRect(0, 0, canvas.width, canvas.height);

    // Draw text (need to reset font after canvas resize)
    context.fillStyle = '#ffffff';
    context.font = 'bold 11px Courier New';
    context.textAlign = 'center';
    context.fillText(labelText, canvas.width / 2, canvas.height / 2 + 4);

    const texture = new THREE.CanvasTexture(canvas);
    const spriteMaterial = new THREE.SpriteMaterial({
        map: texture,
        transparent: true
    });

    node.textSprite = new THREE.Sprite(spriteMaterial);
    const labelPosition = node.position.clone().normalize().multiplyScalar(sphereRadius + 0.5);
    node.textSprite.position.copy(labelPosition);

    // Scale based on canvas width
    const scale = Math.max(1, canvas.width / 128);
    node.textSprite.scale.set(scale, 0.25, 1);

    scene.add(node.textSprite);
}

// Setup mouse controls for camera and node interaction
function setupMouseControls() {
    const canvas = renderer.domElement;

    canvas.addEventListener('mousedown', (event) => {
        isDragging = true;
        previousMousePosition = { x: event.clientX, y: event.clientY };

        // Check for node clicks
        const rect = canvas.getBoundingClientRect();
        mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
        mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

        raycaster.setFromCamera(mouse, camera);
        const meshes = codeNodes.map(node => node.mesh).filter(mesh => mesh);
        const intersects = raycaster.intersectObjects(meshes);

        if (intersects.length > 0) {
            const nodeIndex = intersects[0].object.userData.nodeIndex;
            onNodeClick(nodeIndex);
        }
    });

    canvas.addEventListener('mousemove', (event) => {
        if (!isDragging) return;

        const deltaX = event.clientX - previousMousePosition.x;
        const deltaY = event.clientY - previousMousePosition.y;

        // Rotate camera around origin
        const spherical = new THREE.Spherical();
        spherical.setFromVector3(camera.position);
        spherical.theta -= deltaX * 0.01;
        spherical.phi += deltaY * 0.01;
        spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi));

        camera.position.setFromSpherical(spherical);
        camera.lookAt(0, 0, 0);

        previousMousePosition = { x: event.clientX, y: event.clientY };
    });

    canvas.addEventListener('mouseup', () => {
        isDragging = false;
    });

    // Zoom with mouse wheel
    canvas.addEventListener('wheel', (event) => {
        event.preventDefault();

        const spherical = new THREE.Spherical();
        spherical.setFromVector3(camera.position);
        const delta = event.deltaY > 0 ? 0.5 : -0.5;
        spherical.radius = Math.max(2, Math.min(15, spherical.radius + delta));
        camera.position.setFromSpherical(spherical);
        camera.lookAt(0, 0, 0);
    });
}

// Handle node click
function onNodeClick(nodeIndex) {
    const node = codeNodes[nodeIndex];
    if (!node) return;

    console.log('Node clicked:', node.block);

    // Toggle expansion
    node.isExpanded = !node.isExpanded;

    // Create detailed view if expanded
    if (node.isExpanded) {
        showNodeDetails(node);
    } else {
        hideNodeDetails(node);
    }
}

// Show detailed code block information
function showNodeDetails(node) {
    // For now, log to console
    // In a full implementation, this would show a popup or panel
    console.log('Node details:', {
        type: node.block.type,
        lines: node.block.lines,
        variables: node.block.variables,
        index: node.index
    });

    // Highlight the node
    if (node.mesh) {
        node.mesh.material.emissiveIntensity = 0.5;
    }
}

// Hide node details
function hideNodeDetails(node) {
    if (node.mesh) {
        node.mesh.material.emissiveIntensity = 0.2;
    }
}

// Setup window resize handler
function setupWindowResize() {
    window.addEventListener('resize', () => {
        const container = document.getElementById('globe-canvas').parentElement;
        const width = container.clientWidth;
        const height = container.clientHeight;

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
        renderer.setSize(width, height);
    });
}

// Animation loop
function animate() {
    requestAnimationFrame(animate);

    // Auto-rotation if enabled
    if (isRotating && globeSphere) {
        globeSphere.rotation.y += 0.005;
        globeSphere.rotation.x += 0.002;

        // Rotate nodes with sphere
        codeNodes.forEach(node => {
            if (node.mesh) {
                const quaternion = new THREE.Quaternion();
                quaternion.setFromEuler(new THREE.Euler(0.002, 0.005, 0));
                node.position.applyQuaternion(quaternion);
                node.mesh.position.copy(node.position);

                if (node.textSprite && labelsVisible) {
                    const labelPos = node.position.clone().normalize().multiplyScalar(
                        globeSphere.geometry.parameters.radius + 0.5
                    );
                    node.textSprite.position.copy(labelPos);
                }
            }
        });
    }

    renderer.render(scene, camera);
}

// Clear all nodes and sphere
function clearGlobe() {
    // Remove sphere
    if (globeSphere) {
        scene.remove(globeSphere);
        if (globeSphere.geometry) globeSphere.geometry.dispose();
        if (globeSphere.material) globeSphere.material.dispose();
        globeSphere = null;
    }

    // Remove all nodes
    codeNodes.forEach(node => {
        if (node.mesh) {
            scene.remove(node.mesh);
            if (node.mesh.geometry) node.mesh.geometry.dispose();
            if (node.mesh.material) node.mesh.material.dispose();
        }
        if (node.textSprite) {
            scene.remove(node.textSprite);
            if (node.textSprite.material.map) node.textSprite.material.map.dispose();
            node.textSprite.material.dispose();
        }
        // Remove connection lines
        if (node.connections) {
            node.connections.forEach(line => {
                scene.remove(line);
                if (line.geometry) line.geometry.dispose();
                if (line.material) line.material.dispose();
            });
        }
    });

    codeNodes = [];
}

// Public API functions called from HTML buttons

function resetGlobeView() {
    camera.position.set(0, 0, 8);
    camera.lookAt(0, 0, 0);
    isRotating = false;
    console.log('Globe view reset');
}

function toggleRotation() {
    isRotating = !isRotating;
    console.log('Rotation:', isRotating ? 'ON' : 'OFF');
}

function toggleLabels() {
    labelsVisible = !labelsVisible;

    codeNodes.forEach(node => {
        if (node.textSprite) {
            node.textSprite.visible = labelsVisible;
        }
    });

    console.log('Labels:', labelsVisible ? 'ON' : 'OFF');
}

// Highlight a specific node (used during execution) - darken then return
function highlightNode(nodeIndex) {
    const node = codeNodes[nodeIndex];
    if (!node || !node.mesh) return;

    // Store original values
    const originalColor = node.mesh.material.color.getHex();
    const originalEmissive = node.mesh.material.emissive.getHex();
    const originalEmissiveIntensity = node.mesh.material.emissiveIntensity;

    // Darken the node (reduce brightness)
    node.mesh.material.color.setHex(0x000000);
    node.mesh.material.emissive.setHex(originalColor);
    node.mesh.material.emissiveIntensity = 0.2;

    // Pulse the connected lines
    if (node.connections) {
        node.connections.forEach(line => {
            line.material.opacity = 1.0;
        });
    }

    // Return to normal after 400ms
    setTimeout(() => {
        node.mesh.material.color.setHex(originalColor);
        node.mesh.material.emissive.setHex(originalEmissive);
        node.mesh.material.emissiveIntensity = originalEmissiveIntensity;

        // Fade lines back
        if (node.connections) {
            node.connections.forEach(line => {
                line.material.opacity = 0.6;
            });
        }
    }, 400);
}

// Export functions for use in app.js
window.globeEngine = {
    init: initGlobe,
    createSphere: createGlobeSphere,
    clear: clearGlobe,
    highlightNode: highlightNode,
    reset: resetGlobeView,
    toggleRotation: toggleRotation,
    toggleLabels: toggleLabels
};

console.log('Globe engine loaded');
