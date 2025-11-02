let a, b, c, d, e = [], f = [];
let g = '';
let h = false;
let i = false;
let j = false;
let k = '';
let l = { x: 0, y: 0, z: 8 };
let m = false;
let n = false;
let o = null;
let p = null;
let q = new THREE.Raycaster();
let r = new THREE.Vector2();
let s = [];
let t = [];

class CodeNode {
    constructor(aa, ab, ac) {
        this.block = aa;
        this.position = ab;
        this.index = ac;
        this.isActive = true;
        this.isExpanded = false;
        this.mesh = null;
        this.textSprite = null;
        this.outputSprites = [];
        this.outputLines = [];
        this.nestedNodes = [];
        this.dataConnections = [];
        this.transformedData = null;
        this.outputRing = null;
        this.ringConnection = null;
        this.allOutputs = [];
    }
}

let u = [];
let nodeOutputTracker = {};

function parseCodeBlocks(aa) {
    const ab = [];
    let ac = [0];
    let ad = null;
    let ae = [];
    
    for (let af = 0; af < aa.length; af++) {
        const ag = aa[af];
        const ah = ag.trim();
        
        if (ah.length === 0 || ah.startsWith('#')) continue;
        
        const ai = ag.length - ag.trimStart().length;
        
        const aj = /:\s*$/.test(ah) || 
                   /^\s*(def |class |if |elif |else:|for |while |try:|except|with |@)/i.test(ag);
        
        while (ac.length > 1 && ai < ac[ac.length - 1]) {
            ac.pop();
            if (ad && ad.lines.length > 0) {
                ab.push(ad);
                ad = null;
            }
            if (ae.length > 0) ae.pop();
        }
        
        if (aj) {
            if (ad && ad.lines.length > 0) {
                ab.push(ad);
            }
            
            ad = {
                startLine: af,
                lines: [ag],
                indentLevel: ai,
                type: getBlockType(ag),
                parentIndex: ae.length > 0 ? ae[ae.length - 1] : null,
                variables: extractVariables(ag),
                functionCalls: extractFunctionCalls(ag),
                dataValues: extractDataValues(ag)
            };
            
            if (/:\s*$/.test(ah)) {
                ac.push(ai + 4);
                if (ad.type === 'function') {
                    ae.push(ab.length);
                }
            }
        } else if (ad && ai >= ac[ac.length - 1]) {
            ad.lines.push(ag);
            ad.variables = [...(ad.variables || []), ...extractVariables(ag)];
            ad.functionCalls = [...(ad.functionCalls || []), ...extractFunctionCalls(ag)];
            ad.dataValues = [...(ad.dataValues || []), ...extractDataValues(ag)];
        } else {
            if (ad && ad.lines.length > 0) {
                ab.push(ad);
            }
            ad = {
                startLine: af,
                lines: [ag],
                indentLevel: ai,
                type: 'single',
                parentIndex: null,
                variables: extractVariables(ag),
                functionCalls: extractFunctionCalls(ag),
                dataValues: extractDataValues(ag)
            };
        }
    }
    
    if (ad && ad.lines.length > 0) {
        ab.push(ad);
    }
    
    return ab;
}

function extractVariables(aa) {
    const ab = [];
    const ac = aa.match(/(\w+)\s*=/g);
    if (ac) {
        ac.forEach(ad => {
            const ae = ad.replace(/\s*=/, '');
            ab.push(ae);
        });
    }
    return ab;
}

function extractFunctionCalls(aa) {
    const ab = [];
    const ac = aa.match(/(\w+)\s*\(/g);
    if (ac) {
        ac.forEach(ad => {
            const ae = ad.replace(/\s*\(/, '');
            if (!['print', 'len', 'range', 'str', 'int', 'float', 'list', 'dict'].includes(ae)) {
                ab.push(ae);
            }
        });
    }
    return ab;
}

function extractDataValues(aa) {
    const ab = [];
    const ac = aa.match(/(\w+)\s*=\s*(.+)/);
    if (ac) {
        const ad = ac[1];
        const ae = ac[2].trim();
        if (ae.length > 50) {
            ab.push(`${ad}: ${ae.substring(0, 47)}...`);
        } else {
            ab.push(`${ad}: ${ae}`);
        }
    }
    return ab;
}

function getBlockType(aa) {
    const ab = aa.trim();
    if (/^\s*def /.test(aa)) return 'function';
    if (/^\s*(if |elif )/.test(aa)) return 'condition';
    if (/^\s*else:/.test(aa)) return 'condition';
    if (/^\s*(for |while )/.test(aa)) return 'loop';
    if (/^\s*class /.test(aa)) return 'class';
    if (/^\s*(try:|except|with )/.test(aa)) return 'exception';
    if (/^\s*@/.test(aa)) return 'decorator';
    return 'single';
}

function init() {
    initializeSkulpt();
    
    a = new THREE.Scene();
    a.background = new THREE.Color(0x0a0a0a);
    
    b = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    b.position.set(0, 0, 8);
    l = { x: 0, y: 0, z: 8 };
    
    c = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    c.setSize(window.innerWidth, window.innerHeight);
    c.shadowMap.enabled = true;
    c.shadowMap.type = THREE.PCFSoftShadowMap;
    document.getElementById('container').appendChild(c.domElement);
    
    const aa = new THREE.AmbientLight(0x404040, 0.4);
    a.add(aa);
    
    const ab = new THREE.DirectionalLight(0xffffff, 0.8);
    ab.position.set(5, 5, 5);
    ab.castShadow = true;
    a.add(ab);
    
    const ac = new THREE.PointLight(0x4a90e2, 0.5);
    ac.position.set(-5, 0, 5);
    a.add(ac);
    
    const ad = document.getElementById('editor-input');
    ad.addEventListener('input', handleCodeInput);
    ad.addEventListener('keydown', handleKeyDown);
    
    setupCameraControls();
    setupKeyboardControls();
    setupMouseControls();
    animate();
}

function initializeSkulpt() {
    if (typeof Sk !== 'undefined') {
        m = true;
        console.log('Skulpt loaded successfully');
        document.getElementById('loading').style.display = 'none';
    } else {
        console.log('Skulpt not loaded yet, retrying...');
        setTimeout(initializeSkulpt, 100);
    }
}

function handleCodeInput(aa) {
    g = aa.target.value;
    updateUI();
    
    if (g.length > 0) {
        createCodeSphere(g);
    } else {
        clearSphere();
    }
}

function handleKeyDown(aa) {
    if (aa.key === 'Tab') {
        aa.preventDefault();
        const ab = aa.target.selectionStart;
        const ac = aa.target.selectionEnd;
        aa.target.value = aa.target.value.substring(0, ab) + 
                            '    ' + aa.target.value.substring(ac);
        aa.target.selectionStart = aa.target.selectionEnd = ab + 4;
        handleCodeInput(aa);
    }
}

function setupKeyboardControls() {
    document.addEventListener('keydown', (aa) => {
        if (aa.target.tagName === 'TEXTAREA') return;
        
        const ab = new THREE.Spherical();
        ab.setFromVector3(b.position);
        
        switch(aa.key) {
            case 'ArrowUp':
                ab.radius = Math.max(1.5, ab.radius - 0.3);
                b.position.setFromSpherical(ab);
                break;
            case 'ArrowDown':
                ab.radius = Math.min(15, ab.radius + 0.3);
                b.position.setFromSpherical(ab);
                break;
            case 'ArrowLeft':
                ab.theta -= 0.05;
                b.position.setFromSpherical(ab);
                break;
            case 'ArrowRight':
                ab.theta += 0.05;
                b.position.setFromSpherical(ab);
                break;
        }
        
        b.lookAt(0, 0, 0);
    });
}

function setupMouseControls() {
    let aa = false;
    let ab = { x: 0, y: 0 };
    
    document.addEventListener('mousedown', (ac) => {
        if (ac.target.tagName === 'TEXTAREA' || ac.target.tagName === 'BUTTON') return;
        
        aa = true;
        
        const ad = c.domElement.getBoundingClientRect();
        r.x = ((ac.clientX - ad.left) / ad.width) * 2 - 1;
        r.y = -((ac.clientY - ad.top) / ad.height) * 2 + 1;
        
        q.setFromCamera(r, b);
        
        const ae = u.map(af => af.mesh).filter(af => af);
        const ag = q.intersectObjects(ae);
        
        if (ag.length > 0) {
            const ah = ag[0].object;
            const ai = u.find(af => af.mesh === ah);
            
            if (ac.button === 0) {
                toggleNodeExpansion(ai);
            } else if (ac.button === 2) {
                ac.preventDefault();
                toggleNodeActive(ai);
            }
            
            o = ai;
            n = true;
        }
        
        ab = { x: ac.clientX, y: ac.clientY };
    });
    
    document.addEventListener('mousemove', (ac) => {
        if (aa && !n) {
            const ad = ac.clientX - ab.x;
            const ae = ac.clientY - ab.y;
            
            const af = new THREE.Spherical();
            af.setFromVector3(b.position);
            af.theta -= ad * 0.01;
            af.phi += ae * 0.01;
            af.phi = Math.max(0.1, Math.min(Math.PI - 0.1, af.phi));
            
            b.position.setFromSpherical(af);
            b.lookAt(0, 0, 0);
        } else if (n && o) {
            const ad = c.domElement.getBoundingClientRect();
            r.x = ((ac.clientX - ad.left) / ad.width) * 2 - 1;
            r.y = -((ac.clientY - ad.top) / ad.height) * 2 + 1;
            
            q.setFromCamera(r, b);
            const ae = d ? d.geometry.parameters.radius : 3;
            const af = new THREE.Vector3();
            
            const ag = q.ray;
            const ah = ag.distanceToPoint(new THREE.Vector3(0, 0, 0));
            
            if (ah <= ae) {
                const ai = -ag.origin.dot(ag.direction);
                af.copy(ag.direction).multiplyScalar(ai).add(ag.origin);
                af.normalize().multiplyScalar(ae);
                
                o.position = af.clone();
                o.mesh.position.copy(af);
                
                if (o.isExpanded && o.textSprite) {
                    const aj = af.clone().normalize().multiplyScalar(ae + 0.8);
                    o.textSprite.position.copy(aj);
                }
                
                updateNodeRingPosition(o);
                updateDataFlowLines();
            }
        }
        
        ab = { x: ac.clientX, y: ac.clientY };
    });
    
    document.addEventListener('mouseup', () => {
        aa = false;
        n = false;
        o = null;
    });
    
    document.addEventListener('wheel', (aa) => {
        if (aa.target.tagName === 'TEXTAREA') return;
        
        aa.preventDefault();
        
        const ab = new THREE.Spherical();
        ab.setFromVector3(b.position);
        const ac = aa.deltaY > 0 ? 0.3 : -0.3;
        ab.radius = Math.max(1.5, Math.min(15, ab.radius + ac));
        b.position.setFromSpherical(ab);
        
        b.lookAt(0, 0, 0);
    });
    
    document.addEventListener('contextmenu', (aa) => {
        aa.preventDefault();
    });
}

function updateUI() {
    const aa = g.split('\n').length;
    const ab = g.length;
    document.getElementById('line-count').textContent = aa;
    document.getElementById('char-count').textContent = ab;
}

function createCodeSphere(aa) {
    clearSphere();
    
    const ab = aa.split('\n');
    const ac = parseCodeBlocks(ab);
    
    const ad = Math.max(...ab.map(ae => ae.length));
    const af = Math.max(2, Math.min(6, ad * 0.15));
    
    const ag = new THREE.SphereGeometry(af, 64, 32);
    const ah = new THREE.MeshPhongMaterial({ 
        color: i ? 0x2d5aa0 : 0x4a90e2,
        transparent: true, 
        opacity: 0.3,
        wireframe: false
    });
    
    d = new THREE.Mesh(ag, ah);
    d.castShadow = true;
    d.receiveShadow = true;
    a.add(d);
    
    const ai = new THREE.SphereGeometry(af + 0.01, 32, 16);
    const aj = new THREE.MeshBasicMaterial({ 
        color: 0x666666, 
        wireframe: true,
        transparent: true,
        opacity: 0.2
    });
    const ak = new THREE.Mesh(ai, aj);
    d.add(ak);
    
    createNodesOnSphere(ac, af);
}

function createNodesOnSphere(aa, ab) {
    console.log('Creating nodes on sphere, blocks:', aa.length);
    
    if (aa.length === 0) return;
    
    u = [];
    
    const ac = Math.PI * (3 - Math.sqrt(5));
    
    aa.forEach((ad, ae) => {
        const af = 1 - (ae / (aa.length - 1)) * 2;
        const ag = Math.sqrt(1 - af * af);
        const ah = ac * ae;
        
        const ai = Math.cos(ah) * ag * ab;
        const aj = Math.sin(ah) * ag * ab;
        const ak = af * ab;
        
        const al = new THREE.Vector3(ai, ak, aj);
        const am = new CodeNode(ad, al, ae);
        
        createNodeMesh(am, ab);
        
        if (ad.type === 'function') {
            createNestedNodes(am, aa, ab);
        }
        
        u.push(am);
    });
}

function createNestedNodes(aa, ab, ac) {
    const ad = aa.index;
    
    const ae = ab.filter((af, ag) => 
        ag > ad && 
        af.parentIndex === ad &&
        (af.type === 'loop' || af.type === 'condition')
    );
    
    ae.forEach((af, ag) => {
        const ah = (ag / ae.length) * Math.PI * 2;
        const ai = new THREE.Vector3(
            Math.cos(ah) * 0.3,
            Math.sin(ah) * 0.2,
            Math.sin(ah + Math.PI/2) * 0.3
        );
        
        const aj = aa.position.clone().add(ai);
        
        const ak = new THREE.SphereGeometry(0.08, 12, 12);
        const al = af.type === 'loop' ? 0xff6b6b : 0x4ecdc4;
        const am = new THREE.MeshPhongMaterial({ 
            color: al,
            shininess: 100
        });
        
        const an = new THREE.Mesh(ak, am);
        an.position.copy(aj);
        a.add(an);
        
        aa.nestedNodes.push({
            mesh: an,
            block: af,
            position: aj
        });
        
        const ao = new THREE.BufferGeometry().setFromPoints([
            aa.position, aj
        ]);
        const ap = new THREE.LineBasicMaterial({ 
            color: 0x888888, 
            transparent: true, 
            opacity: 0.6 
        });
        const aq = new THREE.Line(ao, ap);
        a.add(aq);
        s.push(aq);
    });
}

function analyzeDataFlow(aa) {
    clearDataFlowLines();
    
    aa.forEach((ab, ac) => {
        const ad = u[ac];
        if (!ad || !ad.isActive) return;
        
        if (ab.functionCalls && ab.functionCalls.length > 0) {
            ab.functionCalls.forEach(ae => {
                const af = aa.findIndex(ag => 
                    ag.type === 'function' && 
                    ag.lines[0].includes(`def ${ae}`)
                );
                
                if (af !== -1 && af !== ac) {
                    const ag = ab.dataValues && ab.dataValues.length > 0 ? 
                        ab.dataValues[0] : ae;
                    createDataFlowConnection(u[af], ad, ag);
                }
            });
        }
        
        if (ab.variables && ab.variables.length > 0) {
            ab.variables.forEach(ae => {
                aa.forEach((af, ag) => {
                    if (ag !== ac && 
                        af.lines.some(ah => ah.includes(ae) && !ah.includes(`${ae} =`))) {
                        
                        const ai = aa[ac];
                        const aj = ai.dataValues && ai.dataValues.length > 0 ? 
                            ai.dataValues[0] : ae;
                        createDataFlowConnection(ad, u[ag], aj);
                    }
                });
            });
        }
    });
}

function createDataFlowConnection(aa, ab, ac) {
    if (!aa || !ab || aa === ab) return;
    
    const ad = aa.position.clone();
    const ae = ab.position.clone();
    const af = d ? d.geometry.parameters.radius : 3;
    
    const ag = ad.clone().add(ae).multiplyScalar(0.5);
    ag.normalize().multiplyScalar(af * 1.2);
    
    const ah = new THREE.QuadraticBezierCurve3(ad, ag, ae);
    const ai = ah.getPoints(30);
    
    const aj = new THREE.BufferGeometry().setFromPoints(ai);
    const ak = new THREE.LineBasicMaterial({ 
        color: 0xffaa00, 
        transparent: true, 
        opacity: 0.8,
        linewidth: 3
    });
    
    const al = new THREE.Line(aj, ak);
    a.add(al);
    s.push(al);
    
    createTransformationLabel(ag, ac);
    
    aa.dataConnections.push({
        to: ab,
        data: ac,
        line: al
    });
}

function createTransformationLabel(aa, ab) {
    const ac = document.createElement('canvas');
    const ad = ac.getContext('2d');
    
    ac.width = Math.max(120, ab.length * 6 + 20);
    ac.height = 28;
    
    ad.fillStyle = 'rgba(255, 170, 0, 0.95)';
    ad.fillRect(0, 0, ac.width, ac.height);
    
    ad.strokeStyle = '#ff8800';
    ad.lineWidth = 2;
    ad.strokeRect(0, 0, ac.width, ac.height);
    
    ad.fillStyle = '#000000';
    ad.font = 'bold 11px Courier New';
    ad.textAlign = 'center';
    ad.fillText(ab, ac.width / 2, ac.height / 2 + 4);
    
    const ae = new THREE.CanvasTexture(ac);
    const af = new THREE.SpriteMaterial({ 
        map: ae,
        transparent: true,
        alphaTest: 0.1
    });
    
    const ag = new THREE.Sprite(af);
    ag.position.copy(aa);
    ag.scale.set(1.2, 0.3, 1);
    
    a.add(ag);
    t.push(ag);
}

function createNodeMesh(aa, ab) {
    const ac = new THREE.SphereGeometry(0.15, 16, 16);
    
    const ad = {
        'function': 0xffc107,
        'condition': 0x9c27b0,
        'loop': 0xf44336,
        'class': 0x2196f3,
        'exception': 0xff9800,
        'decorator': 0x4caf50,
        'single': 0x9e9e9e
    };
    
    const ae = ad[aa.block.type] || ad['single'];
    const af = aa.isActive ? ae : 0x555555;
    
    const ag = new THREE.MeshPhongMaterial({ 
        color: af,
        shininess: 100
    });
    
    aa.mesh = new THREE.Mesh(ac, ag);
    aa.mesh.position.copy(aa.position);
    aa.mesh.castShadow = true;
    aa.mesh.receiveShadow = true;
    
    a.add(aa.mesh);
}

function toggleNodeExpansion(aa) {
    aa.isExpanded = !aa.isExpanded;
    
    if (aa.isExpanded) {
        if (!aa.textSprite) {
            aa.textSprite = createBlockSprite(aa.block, aa.index);
            const ab = aa.position.clone().normalize().multiplyScalar(d.geometry.parameters.radius + 0.8);
            aa.textSprite.position.copy(ab);
            a.add(aa.textSprite);
        }
        aa.textSprite.visible = true;
    } else {
        if (aa.textSprite) {
            aa.textSprite.visible = false;
        }
    }
}

function toggleNodeActive(aa) {
    aa.isActive = !aa.isActive;
    
    const ab = {
        'function': 0xffc107,
        'condition': 0x9c27b0,
        'loop': 0xf44336,
        'class': 0x2196f3,
        'exception': 0xff9800,
        'decorator': 0x4caf50,
        'single': 0x9e9e9e
    };
    
    const ac = ab[aa.block.type] || ab['single'];
    const ad = aa.isActive ? ac : 0x555555;
    
    if (aa.mesh) {
        aa.mesh.material.color.setHex(ad);
    }
}

function createBlockSprite(aa, ab) {
    const ac = document.createElement('canvas');
    const ad = ac.getContext('2d');
    
    const ae = Math.max(...aa.lines.map(af => af.length));
    const ag = Math.max(250, ae * 8);
    const ah = Math.max(60, aa.lines.length * 20 + 25);
    
    ac.width = ag;
    ac.height = ah;
    
    const ai = {
        'function': 'rgba(255, 193, 7, 0.9)',
        'condition': 'rgba(156, 39, 176, 0.9)',
        'loop': 'rgba(244, 67, 54, 0.9)',
        'class': 'rgba(33, 150, 243, 0.9)',
        'exception': 'rgba(255, 152, 0, 0.9)',
        'decorator': 'rgba(76, 175, 80, 0.9)',
        'single': 'rgba(158, 158, 158, 0.8)'
    };
    
    ad.fillStyle = ai[aa.type] || ai['single'];
    ad.fillRect(0, 0, ac.width, ac.height);
    
    ad.strokeStyle = '#ffffff';
    ad.lineWidth = 2;
    ad.strokeRect(0, 0, ac.width, ac.height);
    
    ad.font = 'bold 12px Courier New';
    ad.textAlign = 'left';
    ad.fillStyle = '#000000';
    
    ad.fillText(`${aa.type.toUpperCase()} ${ab + 1}`, 6, 16);
    
    aa.lines.forEach((aj, ak) => {
        const al = aj.length - aj.trimStart().length;
        const am = '•'.repeat(Math.floor(al / 4));
        const an = am + aj.trimStart();
        ad.fillText(an, 6, 32 + (ak * 16));
    });
    
    const ao = new THREE.CanvasTexture(ac);
    ao.needsUpdate = true;
    
    const ap = new THREE.SpriteMaterial({ 
        map: ao,
        transparent: true,
        alphaTest: 0.1
    });
    
    const aq = new THREE.Sprite(ap);
    const ar = aa.type === 'function' ? 2 : 1.5;
    const as = Math.max(0.3, aa.lines.length * 0.1);
    aq.scale.set(ar, as, 1);
    
    return aq;
}

async function executePythonCode() {
    if (!m) {
        return 'Python interpreter not ready yet. Please wait a moment and try again.';
    }
    
    u.forEach(aa => {
        aa.allOutputs = [];
    });
    nodeOutputTracker = {};
    
    const ab = u
        .filter(aa => aa.isActive)
        .map(aa => aa.block.lines.join('\n'))
        .join('\n');
    
    return new Promise((ac) => {
        let ad = '';
        let ae = [];
        let currentNode = null;
        
        try {
            Sk.pre = "output";
            Sk.configure({
                output: function(af) {
                    ad += af;
                    const ag = af.split('\n').filter(ah => ah.trim().length > 0);
                    
                    ag.forEach(output => {
                        const nodeIndex = findNodeForOutput(output);
                        if (nodeIndex !== -1) {
                            if (!nodeOutputTracker[nodeIndex]) {
                                nodeOutputTracker[nodeIndex] = [];
                            }
                            nodeOutputTracker[nodeIndex].push(output);
                        }
                    });
                    
                    ae.push(...ag);
                },
                read: function(af) {
                    if (Sk.builtinFiles === undefined || Sk.builtinFiles["files"][af] === undefined) {
                        throw "File not found: '" + af + "'";
                    }
                    return Sk.builtinFiles["files"][af];
                }
            });
            
            const af = Sk.misceval.asyncToPromise(function() {
                return Sk.importMainWithBody("<stdin>", false, ab, true);
            });
            
            af.then(
                function(ag) {
                    distributeOutputsToNodes();
                    ac(ad || 'Code executed successfully (no output)');
                },
                function(ag) {
                    ac(`Python Error: ${ag.toString()}`);
                }
            );
            
        } catch (af) {
            ac(`Error: ${af.toString()}`);
        }
    });
}

function findNodeForOutput(output) {
    for (let i = u.length - 1; i >= 0; i--) {
        const node = u[i];
        if (!node.isActive) continue;
        
        const lines = node.block.lines.join(' ').toLowerCase();
        if (lines.includes('print(')) {
            return i;
        }
    }
    return -1;
}

function distributeOutputsToNodes() {
    Object.keys(nodeOutputTracker).forEach(nodeIndex => {
        const node = u[parseInt(nodeIndex)];
        if (node) {
            node.allOutputs = nodeOutputTracker[nodeIndex];
            createOutputRing(node);
        }
    });
}

function createOutputRing(aa) {
    if (aa.allOutputs.length === 0) return;
    
    clearNodeOutputs(aa);
    
    const ab = d ? d.geometry.parameters.radius : 3;
    const ac = 1.5 + (aa.allOutputs.length * 0.1);
    const ad = ab + ac;
    
    const ae = {
        'function': 0xffc107,
        'condition': 0x9c27b0,
        'loop': 0xf44336,
        'class': 0x2196f3,
        'exception': 0xff9800,
        'decorator': 0x4caf50,
        'single': 0x9e9e9e
    };
    
    const af = ae[aa.block.type] || ae['single'];
    
    const ag = new THREE.TorusGeometry(ad, 0.05, 8, 64);
    const ah = new THREE.MeshBasicMaterial({ 
        color: af,
        transparent: true,
        opacity: 0.6
    });
    
    aa.outputRing = new THREE.Mesh(ag, ah);
    
    const ai = aa.position.clone().normalize();
    const aj = new THREE.Vector3(0, 1, 0);
    const ak = ai.clone().cross(aj).normalize();
    const al = ak.clone().cross(ai).normalize();
    
    const am = new THREE.Matrix4();
    am.makeBasis(ak, al, ai);
    aa.outputRing.setRotationFromMatrix(am);
    aa.outputRing.position.copy(ai.clone().multiplyScalar(ab));
    
    a.add(aa.outputRing);
    
    const an = ai.clone().multiplyScalar(ad);
    const ao = new THREE.BufferGeometry().setFromPoints([aa.position, an]);
    const ap = new THREE.LineBasicMaterial({ 
        color: af, 
        transparent: true, 
        opacity: 0.8 
    });
    aa.ringConnection = new THREE.Line(ao, ap);
    a.add(aa.ringConnection);
    
    const aq = aa.allOutputs.length;
    aa.allOutputs.forEach((ar, as) => {
        const at = (as / aq) * Math.PI * 2;
        
        const au = ak.clone().multiplyScalar(Math.cos(at) * ad)
            .add(al.clone().multiplyScalar(Math.sin(at) * ad))
            .add(ai.clone().multiplyScalar(ab));
        
        const av = createOutputSprite(ar, aa.block.type);
        av.position.copy(au);
        
        const aw = ar.length;
        const ax = Math.min(1.2, Math.max(0.4, aw * 0.03));
        av.scale.set(ax, 0.25, 1);
        
        a.add(av);
        aa.outputSprites.push(av);
    });
}

function updateNodeRingPosition(aa) {
    if (!aa.outputRing) return;
    
    const ab = d ? d.geometry.parameters.radius : 3;
    const ac = aa.position.clone().normalize();
    const ad = new THREE.Vector3(0, 1, 0);
    const ae = ac.clone().cross(ad).normalize();
    const af = ae.clone().cross(ac).normalize();
    
    const ag = new THREE.Matrix4();
    ag.makeBasis(ae, af, ac);
    aa.outputRing.setRotationFromMatrix(ag);
    aa.outputRing.position.copy(ac.clone().multiplyScalar(ab));
    
    if (aa.ringConnection) {
        const ah = 1.5 + (aa.allOutputs.length * 0.1);
        const ai = ab + ah;
        const aj = ac.clone().multiplyScalar(ai);
        aa.ringConnection.geometry.setFromPoints([aa.position, aj]);
        aa.ringConnection.geometry.attributes.position.needsUpdate = true;
    }
    
    const ak = 1.5 + (aa.allOutputs.length * 0.1);
    const al = ab + ak;
    const am = aa.allOutputs.length;
    
    aa.outputSprites.forEach((an, ao) => {
        const ap = (ao / am) * Math.PI * 2;
        
        const aq = ae.clone().multiplyScalar(Math.cos(ap) * al)
            .add(af.clone().multiplyScalar(Math.sin(ap) * al))
            .add(ac.clone().multiplyScalar(ab));
        
        an.position.copy(aq);
    });
}

function clearNodeOutputs(aa) {
    if (aa.outputRing) {
        a.remove(aa.outputRing);
        if (aa.outputRing.geometry) aa.outputRing.geometry.dispose();
        if (aa.outputRing.material) aa.outputRing.material.dispose();
        aa.outputRing = null;
    }
    
    if (aa.ringConnection) {
        a.remove(aa.ringConnection);
        if (aa.ringConnection.geometry) aa.ringConnection.geometry.dispose();
        if (aa.ringConnection.material) aa.ringConnection.material.dispose();
        aa.ringConnection = null;
    }
    
    aa.outputSprites.forEach(ab => {
        a.remove(ab);
        if (ab.material.map) ab.material.map.dispose();
        ab.material.dispose();
    });
    aa.outputSprites = [];
}

function createOutputSprite(aa, ab = 'single') {
    const ac = document.createElement('canvas');
    const ad = ac.getContext('2d');
    
    const ae = aa.length;
    ac.width = Math.max(120, ae * 7 + 16);
    ac.height = 32;
    
    ad.fillStyle = 'rgba(0, 255, 0, 0.9)';
    ad.strokeStyle = '#00aa00';
    
    ad.fillRect(0, 0, ac.width, ac.height);
    ad.lineWidth = 2;
    ad.strokeRect(0, 0, ac.width, ac.height);
    
    ad.fillStyle = '#000000';
    ad.font = 'bold 12px Courier New';
    ad.textAlign = 'center';
    ad.fillText(aa, ac.width / 2, ac.height / 2 + 4);
    
    const af = new THREE.CanvasTexture(ac);
    af.needsUpdate = true;
    
    const ag = new THREE.SpriteMaterial({ 
        map: af,
        transparent: true,
        alphaTest: 0.1
    });
    
    return new THREE.Sprite(ag);
}

function updateDataFlowLines() {
    u.forEach(aa => {
        if (aa.dataConnections && aa.dataConnections.length > 0) {
            aa.dataConnections.forEach(ab => {
                if (ab.line && ab.to) {
                    const ac = aa.position.clone();
                    const ad = ab.to.position.clone();
                    const ae = d ? d.geometry.parameters.radius : 3;
                    
                    const af = ac.clone().add(ad).multiplyScalar(0.5);
                    af.normalize().multiplyScalar(ae * 1.2);
                    
                    const ag = new THREE.QuadraticBezierCurve3(ac, af, ad);
                    const ah = ag.getPoints(30);
                    
                    ab.line.geometry.setFromPoints(ah);
                    ab.line.geometry.attributes.position.needsUpdate = true;
                }
            });
        }
    });
    
    u.forEach(aa => {
        if (aa.nestedNodes && aa.nestedNodes.length > 0) {
            aa.nestedNodes.forEach((ab, ac) => {
                if (s[ac]) {
                    const ad = s[ac].geometry;
                    ad.setFromPoints([aa.position, ab.position]);
                    ad.attributes.position.needsUpdate = true;
                }
            });
        }
    });
}

function clearDataFlowLines() {
    s.forEach(aa => {
        a.remove(aa);
        if (aa.material) aa.material.dispose();
        if (aa.geometry) aa.geometry.dispose();
    });
    s = [];
    
    t.forEach(aa => {
        a.remove(aa);
        if (aa.material.map) aa.material.map.dispose();
        aa.material.dispose();
    });
    t = [];
    
    u.forEach(aa => {
        aa.dataConnections = [];
    });
}

async function runCode() {
    if (!m) {
        k = 'Python interpreter is still loading. Please wait and try again.';
        return;
    }
    
    try {
        k = await executePythonCode();
        
        const aa = g.split('\n');
        const ab = parseCodeBlocks(aa);
        analyzeDataFlow(ab);
        
    } catch (aa) {
        k = `Execution Error: ${aa.message}`;
    }
    
    console.log('Code output:', k);
}

function clearOutputDisplay() {
    u.forEach(aa => {
        clearNodeOutputs(aa);
        aa.allOutputs = [];
    });
    
    f.forEach(aa => {
        a.remove(aa);
        if (aa.material) {
            if (aa.material.map) aa.material.map.dispose();
            aa.material.dispose();
        }
        if (aa.geometry) aa.geometry.dispose();
    });
    f = [];
}

function clearSphere() {
    if (d) {
        a.remove(d);
        d = null;
    }
    
    u.forEach(aa => {
        if (aa.mesh) {
            a.remove(aa.mesh);
            if (aa.mesh.material) aa.mesh.material.dispose();
            if (aa.mesh.geometry) aa.mesh.geometry.dispose();
        }
        if (aa.textSprite) {
            a.remove(aa.textSprite);
            if (aa.textSprite.material.map) aa.textSprite.material.map.dispose();
            aa.textSprite.material.dispose();
        }
        clearNodeOutputs(aa);
        
        aa.nestedNodes.forEach(ab => {
            a.remove(ab.mesh);
            if (ab.mesh.material) ab.mesh.material.dispose();
            if (ab.mesh.geometry) ab.mesh.geometry.dispose();
        });
    });
    
    u = [];
    
    e.forEach(aa => {
        a.remove(aa);
        if (aa.material.map) {
            aa.material.map.dispose();
        }
        aa.material.dispose();
    });
    e = [];
    
    clearOutputDisplay();
    clearDataFlowLines();
}

function toggleSyntaxHighlight() {
    i = !i;
    if (g) {
        createCodeSphere(g);
    }
}

function rotateSphere() {
    h = !h;
}

function resetView() {
    b.position.set(0, 0, 8);
    b.lookAt(0, 0, 0);
    h = false;
    clearOutputDisplay();
    clearDataFlowLines();
    
    u.forEach(aa => {
        aa.isActive = true;
        aa.isExpanded = false;
        if (aa.textSprite) aa.textSprite.visible = false;
        
        const ab = {
            'function': 0xffc107,
            'condition': 0x9c27b0,
            'loop': 0xf44336,
            'class': 0x2196f3,
            'exception': 0xff9800,
            'decorator': 0x4caf50,
            'single': 0x9e9e9e
        };
        
        const ac = ab[aa.block.type] || ab['single'];
        if (aa.mesh) {
            aa.mesh.material.color.setHex(ac);
        }
    });
}

function setupCameraControls() {
}

function animate() {
    requestAnimationFrame(animate);
    
    if (d && h) {
        d.rotation.y += 0.01;
        d.rotation.x += 0.005;
        
        u.forEach(aa => {
            if (aa.mesh) {
                const ab = new THREE.Quaternion();
                ab.setFromEuler(new THREE.Euler(0.005, 0.01, 0));
                aa.position.applyQuaternion(ab);
                aa.mesh.position.copy(aa.position);
                
                if (aa.isExpanded && aa.textSprite) {
                    const ac = aa.position.clone().normalize().multiplyScalar(d.geometry.parameters.radius + 0.8);
                    aa.textSprite.position.copy(ac);
                }
                
                updateNodeRingPosition(aa);
                
                aa.nestedNodes.forEach(ad => {
                    ad.position.applyQuaternion(ab);
                    ad.mesh.position.copy(ad.position);
                });
            }
        });
        
        updateDataFlowLines();
    }
    
    c.render(a, b);
}

window.addEventListener('resize', () => {
    b.aspect = window.innerWidth / window.innerHeight;
    b.updateProjectionMatrix();
    c.setSize(window.innerWidth, window.innerHeight);
});

window.addEventListener('load', init);