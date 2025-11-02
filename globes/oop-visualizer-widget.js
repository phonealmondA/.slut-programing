const { ReactWidget } = require('@theia/core/lib/browser');
const React = require('react');

class OOPVisualizerWidget extends ReactWidget {
    static get ID() { return 'oop-visualizer-widget'; }
    static get LABEL() { return 'OOP Sphere Visualizer'; }

    constructor() {
        super();
        this.id = OOPVisualizerWidget.ID;
        this.title.label = OOPVisualizerWidget.LABEL;
        this.title.caption = OOPVisualizerWidget.LABEL;
        this.title.closable = true;
        this.sphereInitialized = false;
    }

    onAfterAttach() {
        super.onAfterAttach();
        if (!this.sphereInitialized) {
            setTimeout(() => {
                this.initializeSphereVisualization();
            }, 100);
            this.sphereInitialized = true;
        }
    }

    initializeSphereVisualization() {
        const container = this.node.querySelector('#sphere-container');
        if (container) {
            this.loadExternalLibraries().then(() => {
                this.initBasicSphere(container);
            });
        }
    }

    loadExternalLibraries() {
        return new Promise((resolve) => {
            if (window.THREE && window.Sk) {
                resolve();
                return;
            }
            
            const loadThreeJS = new Promise((res) => {
                if (window.THREE) {
                    res();
                    return;
                }
                const script = document.createElement('script');
                script.src = 'https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js';
                script.onload = res;
                document.head.appendChild(script);
            });

            const loadSkulpt = new Promise((res) => {
                if (window.Sk) {
                    res();
                    return;
                }
                const script1 = document.createElement('script');
                script1.src = 'https://unpkg.com/skulpt@0.11.1/dist/skulpt.min.js';
                script1.onload = () => {
                    const script2 = document.createElement('script');
                    script2.src = 'https://unpkg.com/skulpt@0.11.1/dist/skulpt-stdlib.js';
                    script2.onload = res;
                    document.head.appendChild(script2);
                };
                document.head.appendChild(script1);
            });

            Promise.all([loadThreeJS, loadSkulpt]).then(resolve);
        });
    }

    initBasicSphere(container) {
        // Basic Three.js scene setup
        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x0a0a0a);
        
        const camera = new THREE.PerspectiveCamera(75, container.clientWidth / container.clientHeight, 0.1, 1000);
        camera.position.set(0, 0, 8);
        
        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(container.clientWidth, container.clientHeight);
        container.appendChild(renderer.domElement);
        
        // Add lighting
        const ambientLight = new THREE.AmbientLight(0x404040, 0.4);
        scene.add(ambientLight);
        
        const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
        directionalLight.position.set(5, 5, 5);
        scene.add(directionalLight);
        
        // Create the main sphere
        const geometry = new THREE.SphereGeometry(3, 64, 32);
        const material = new THREE.MeshPhongMaterial({ 
            color: 0x4a90e2,
            transparent: true, 
            opacity: 0.3
        });
        
        const sphere = new THREE.Mesh(geometry, material);
        scene.add(sphere);
        
        // Add wireframe
        const wireGeometry = new THREE.SphereGeometry(3.01, 32, 16);
        const wireMaterial = new THREE.MeshBasicMaterial({ 
            color: 0x666666, 
            wireframe: true,
            transparent: true,
            opacity: 0.2
        });
        const wireframe = new THREE.Mesh(wireGeometry, wireMaterial);
        scene.add(wireframe);
        
        // Add some sample nodes
        this.addSampleNodes(scene);
        
        // Camera controls
        this.setupCameraControls(camera, renderer);
        
        // Animation loop
        const animate = () => {
            requestAnimationFrame(animate);
            sphere.rotation.y += 0.005;
            wireframe.rotation.y += 0.005;
            renderer.render(scene, camera);
        };
        animate();
        
        // Handle resize
        const handleResize = () => {
            camera.aspect = container.clientWidth / container.clientHeight;
            camera.updateProjectionMatrix();
            renderer.setSize(container.clientWidth, container.clientHeight);
        };
        
        window.addEventListener('resize', handleResize);
        
        console.log('🌐 Sphere visualization initialized!');
    }
    
    addSampleNodes(scene) {
        // Add some colorful sample nodes to demonstrate the concept
        const nodeColors = {
            'function': 0xffc107,
            'condition': 0x9c27b0,
            'loop': 0xf44336,
            'class': 0x2196f3
        };
        
        const nodeTypes = Object.keys(nodeColors);
        
        for (let i = 0; i < 8; i++) {
            const phi = Math.acos(-1 + (2 * i) / 8);
            const theta = Math.sqrt(8 * Math.PI) * phi;
            
            const x = Math.cos(theta) * Math.sin(phi) * 3.2;
            const y = Math.sin(theta) * Math.sin(phi) * 3.2;
            const z = Math.cos(phi) * 3.2;
            
            const nodeGeometry = new THREE.SphereGeometry(0.15, 16, 16);
            const nodeType = nodeTypes[i % nodeTypes.length];
            const nodeMaterial = new THREE.MeshPhongMaterial({ 
                color: nodeColors[nodeType],
                shininess: 100
            });
            
            const node = new THREE.Mesh(nodeGeometry, nodeMaterial);
            node.position.set(x, y, z);
            scene.add(node);
        }
    }
    
    setupCameraControls(camera, renderer) {
        let mouseDown = false;
        let mousePos = { x: 0, y: 0 };
        
        const canvas = renderer.domElement;
        
        canvas.addEventListener('mousedown', (e) => {
            mouseDown = true;
            mousePos = { x: e.clientX, y: e.clientY };
        });
        
        canvas.addEventListener('mousemove', (e) => {
            if (mouseDown) {
                const deltaX = e.clientX - mousePos.x;
                const deltaY = e.clientY - mousePos.y;
                
                const spherical = new THREE.Spherical();
                spherical.setFromVector3(camera.position);
                spherical.theta -= deltaX * 0.01;
                spherical.phi += deltaY * 0.01;
                spherical.phi = Math.max(0.1, Math.min(Math.PI - 0.1, spherical.phi));
                
                camera.position.setFromSpherical(spherical);
                camera.lookAt(0, 0, 0);
                
                mousePos = { x: e.clientX, y: e.clientY };
            }
        });
        
        canvas.addEventListener('mouseup', () => {
            mouseDown = false;
        });
        
        canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            const spherical = new THREE.Spherical();
            spherical.setFromVector3(camera.position);
            const delta = e.deltaY > 0 ? 0.3 : -0.3;
            spherical.radius = Math.max(1.5, Math.min(15, spherical.radius + delta));
            camera.position.setFromSpherical(spherical);
            camera.lookAt(0, 0, 0);
        });
    }

    render() {
        return React.createElement('div', {
            style: {
                height: '100%',
                backgroundColor: '#1e1e1e',
                color: 'white',
                fontFamily: 'monospace',
                display: 'flex',
                flexDirection: 'column'
            }
        }, [
            React.createElement('div', {
                key: 'header',
                style: { 
                    padding: '10px', 
                    borderBottom: '1px solid #333',
                    background: 'rgba(0,0,0,0.8)'
                }
            }, [
                React.createElement('h3', { 
                    key: 'title', 
                    style: { margin: 0, color: '#4a90e2' } 
                }, '🐍 OOP Sphere Visualizer'),
                React.createElement('p', { 
                    key: 'desc', 
                    style: { margin: '5px 0', fontSize: '12px', color: '#aaa' } 
                }, 'Interactive 3D visualization for Object-Oriented Programming concepts')
            ]),
            React.createElement('div', {
                key: 'sphere-container',
                id: 'sphere-container',
                style: {
                    flex: 1,
                    position: 'relative',
                    overflow: 'hidden'
                }
            })
        ]);
    }
}

module.exports = { OOPVisualizerWidget };