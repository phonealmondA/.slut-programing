// Main Application Logic for Quantum Consciousness IDE
// Integrates Globe Visualization with Tauri Backend

// Global state
let currentFile = null;
let isRunning = false;
let currentBlocks = [];

// Tauri API references
let invoke, open, listen;

// UI Elements
let dropZone, fileInfo, statusIndicator;
let runOnceBtn, runLoopBtn, stopBtn;
let varCount, solCount, accuracy, accuracyBar, obsCount;
let stepsOutput, consoleOutput, codeEditor;

// Wait for Tauri API to be ready
function waitForTauri() {
    return new Promise((resolve) => {
        if (window.__TAURI__ && window.__TAURI__.tauri && window.__TAURI__.dialog && window.__TAURI__.event) {
            resolve();
        } else {
            console.log('Waiting for Tauri API...');
            setTimeout(() => waitForTauri().then(resolve), 100);
        }
    });
}

// Initialize application
window.addEventListener('DOMContentLoaded', async () => {
    console.log('DOM loaded, initializing application...');

    try {
        // Wait for Tauri
        await waitForTauri();
        console.log('Tauri API ready');

        // Initialize Tauri APIs
        invoke = window.__TAURI__.tauri.invoke;
        open = window.__TAURI__.dialog.open;
        listen = window.__TAURI__.event.listen;

        // Initialize UI elements
        initializeUI();

        // Initialize globe visualization
        if (window.globeEngine) {
            window.globeEngine.init();
            console.log('Globe engine initialized');
        } else {
            console.error('Globe engine not loaded');
        }

        // Setup event listeners
        setupFileHandlers();
        setupTauriListeners();

        // Auto-update stats
        setInterval(updateStats, 2000);

        // Initial stats load
        await updateStats();

        console.log('Application initialized successfully');
        addStep('Application ready. Load a .slut file to begin.');

    } catch (error) {
        console.error('Initialization failed:', error);
        alert('Failed to initialize application: ' + error);
    }
});

// Initialize UI element references
function initializeUI() {
    dropZone = document.getElementById('dropZone');
    fileInfo = document.getElementById('fileInfo');
    statusIndicator = document.getElementById('statusIndicator');
    runOnceBtn = document.getElementById('runOnceBtn');
    runLoopBtn = document.getElementById('runLoopBtn');
    stopBtn = document.getElementById('stopBtn');
    varCount = document.getElementById('varCount');
    solCount = document.getElementById('solCount');
    accuracy = document.getElementById('accuracy');
    accuracyBar = document.getElementById('accuracyBar');
    obsCount = document.getElementById('obsCount');
    stepsOutput = document.getElementById('steps-output');
    consoleOutput = document.getElementById('console-output');
    codeEditor = document.getElementById('code-editor');
}

// Setup file drop and selection handlers
function setupFileHandlers() {
    // Click to browse
    dropZone.addEventListener('click', async () => {
        try {
            const selected = await open({
                multiple: false,
                filters: [{
                    name: 'SLUT Files',
                    extensions: ['slut']
                }]
            });

            if (selected) {
                await loadFile(selected);
            }
        } catch (error) {
            console.error('File selection error:', error);
            addStep('Error selecting file: ' + error, 'error');
        }
    });

    // Tauri file drop events
    listen('tauri://file-drop', async (event) => {
        const files = event.payload;
        if (files && files.length > 0) {
            const filePath = files[0];
            if (filePath.endsWith('.slut')) {
                await loadFile(filePath);
                dropZone.classList.remove('drag-over');
            } else {
                addStep('Please drop a .slut file', 'error');
            }
        }
    });

    listen('tauri://file-drop-hover', () => {
        dropZone.classList.add('drag-over');
    });

    listen('tauri://file-drop-cancelled', () => {
        dropZone.classList.remove('drag-over');
    });
}

// Setup Tauri event listeners for real-time updates
function setupTauriListeners() {
    // Listen for console output from backend (for execution steps)
    listen('console-output', (event) => {
        const { message, level } = event.payload;

        // Add to execution steps (summary view)
        addStep(message, level || 'info');

        // For detailed console output, we'll parse the full result after execution
    });

    // Listen for execution progress (if implemented in backend)
    listen('execution-progress', (event) => {
        const { iteration, accuracy: acc } = event.payload;
        if (acc !== undefined) {
            updateAccuracyDisplay(acc);
        }
    });
}

// Load and parse a .slut file
async function loadFile(filePath) {
    console.log('Loading file:', filePath);
    currentFile = filePath;

    try {
        const fileName = filePath.split('\\').pop().split('/').pop();

        // Read file contents using Tauri fs API
        const fileContents = await window.__TAURI__.fs.readTextFile(filePath);

        // Display file contents in code editor
        if (codeEditor) {
            codeEditor.value = fileContents;
        }

        fileInfo.innerHTML = `
            <p><strong>📄 ${fileName}</strong></p>
            <p style="font-size: 0.85em; opacity: 0.7;">File loaded - ${fileContents.split('\n').length} lines</p>
        `;

        addStep(`File loaded: ${fileName}`, 'success');

        // Parse the file into code blocks
        await parseAndVisualize(filePath, fileContents);

        // Enable execute buttons
        runOnceBtn.disabled = false;
        runLoopBtn.disabled = false;

    } catch (error) {
        console.error('Error loading file:', error);
        addStep('Error loading file: ' + error, 'error');

        // Fallback: still show placeholder if file read fails
        if (codeEditor) {
            codeEditor.value = '// Failed to load file\n// Error: ' + error;
        }
    }
}

// Parse .slut file and create visualization
async function parseAndVisualize(filePath, fileContents) {
    try {
        // Parse .slut code into blocks
        currentBlocks = parseSlutCode(fileContents);

        // Visualize in globe
        if (window.globeEngine) {
            window.globeEngine.createSphere(currentBlocks);
            addStep(`Visualized ${currentBlocks.length} code blocks on globe`, 'success');
        }

    } catch (error) {
        console.error('Error parsing file:', error);
        addStep('Error parsing file: ' + error, 'error');
    }
}

// Parse .slut code into code blocks
function parseSlutCode(code) {
    const lines = code.split('\n');
    const blocks = [];

    lines.forEach((line, index) => {
        const trimmedLine = line.trim();

        // Skip empty lines, comments, structural braces, and observe_execution
        if (trimmedLine.length === 0 ||
            trimmedLine.startsWith('#') ||
            trimmedLine === '{' ||
            trimmedLine === '}' ||
            trimmedLine.includes('observe_execution')) {
            return;
        }

        // Determine block type based on actual .slut syntax
        let blockType = 'Unknown';
        let varName = '';

        // Check for function/main declarations (starts with *)
        if (trimmedLine.startsWith('*')) {
            blockType = 'Function';
        }
        // Check for result operations (special handling)
        else if (trimmedLine.includes('result(') || trimmedLine.includes('result[')) {
            blockType = 'Result';
            varName = 'result';
        }
        // Check for target variables
        else if (trimmedLine.includes('target') && trimmedLine.includes('<>')) {
            blockType = 'Target';
            const match = trimmedLine.match(/(\w+)\s*</);
            varName = match ? match[1] : 'target';
        }
        // Check for input variables (firstInput, secondInput, etc.)
        else if ((trimmedLine.includes('Input') || trimmedLine.includes('input')) && trimmedLine.includes('<>')) {
            blockType = 'Input';
            const match = trimmedLine.match(/(\w+)\s*</);
            varName = match ? match[1] : 'input';
        }
        // Check for other variable assignments (uses <> operator)
        else if (trimmedLine.includes('<>')) {
            blockType = 'Variable';
            const match = trimmedLine.match(/(\w+)\s*</);
            varName = match ? match[1] : '';
        }
        // Check for output operations (speak, woof, print)
        else if (trimmedLine.includes('speak(') || trimmedLine.includes('woof ') ||
                 trimmedLine.includes('print(')) {
            blockType = 'Output';
        }
        // Check for loops (while, repeat, for)
        else if (trimmedLine.includes('while ') || trimmedLine.includes('repeat') ||
                 trimmedLine.includes('loop')) {
            blockType = 'Loop';
        }
        // Check for conditionals
        else if (trimmedLine.startsWith('if ') || trimmedLine.startsWith('else')) {
            blockType = 'Condition';
        }
        // Check for quantum operations (starts with ^ or contains observe/collapse)
        else if (trimmedLine.startsWith('^') || trimmedLine.includes('observe') || trimmedLine.includes('collapse')) {
            blockType = 'QuantumOperation';
        }

        blocks.push({
            type: blockType,
            lines: [trimmedLine],
            fullLine: line,
            startLine: index,
            varName: varName  // Store variable name for connection tracking
        });
    });

    // Analyze dependencies and add connection info
    analyzeDependencies(blocks);

    return blocks.length > 0 ? blocks : parseSampleBlocks();
}

// Analyze dependencies between blocks for connection lines
function analyzeDependencies(blocks) {
    // Find all variable definitions
    const variables = {};
    blocks.forEach((block, index) => {
        if (block.varName) {
            variables[block.varName] = index;
        }
    });

    // Find blocks that reference these variables
    blocks.forEach((block, index) => {
        block.connections = [];

        // Check if this block references any variables
        const lineText = block.lines[0];

        // For result blocks, connect to inputs and targets
        if (block.type === 'Result') {
            Object.keys(variables).forEach(varName => {
                if (lineText.includes(varName) && variables[varName] !== index) {
                    block.connections.push(variables[varName]);
                }
            });
        }

        // For other blocks, connect to variables they reference
        else {
            Object.keys(variables).forEach(varName => {
                if (lineText.includes(varName) && variables[varName] !== index) {
                    block.connections.push(variables[varName]);
                }
            });
        }
    });
}

// Simple .slut code block parser (placeholder)
function parseSampleBlocks() {
    // This is a placeholder - in production, this would parse actual .slut code
    // For now, create sample blocks to demonstrate the visualization
    return [
        { type: 'Variable', lines: ['var x = 10'], startLine: 0 },
        { type: 'Variable', lines: ['var target = 42'], startLine: 1 },
        { type: 'QuantumOperation', lines: ['observe x'], startLine: 2 },
        { type: 'MathOperation', lines: ['x = x + 1'], startLine: 3 },
        { type: 'Loop', lines: ['repeat until accurate'], startLine: 4 },
        { type: 'QuantumOperation', lines: ['collapse x'], startLine: 5 },
        { type: 'Condition', lines: ['if x == target'], startLine: 6 },
        { type: 'Output', lines: ['print "Solved!"'], startLine: 7 }
    ];
}

// Execute .slut file once
async function runOnce() {
    if (!currentFile) {
        addStep('No file loaded', 'error');
        return;
    }

    setRunningState(true);
    addStep(`Executing: ${currentFile}`, 'info');
    clearSteps();

    try {
        const result = await invoke('run_file', { filePath: currentFile });

        // Parse result for console output (look for speak() outputs)
        parseExecutionOutput(result);

        addStep('Execution complete: ' + result, 'success');

        // Highlight nodes during execution (simulated)
        simulateExecution();

        await updateStats();

    } catch (error) {
        addStep('Execution error: ' + error, 'error');
        console.error('Execution error:', error);
    } finally {
        setRunningState(false);
    }
}

// Parse execution output to extract console messages
function parseExecutionOutput(result) {
    // Split by newlines and display all output
    const lines = result.split('\n');
    lines.forEach(line => {
        const trimmed = line.trim();
        if (trimmed.length > 0) {
            // Format the line with appropriate styling based on prefix
            addFormattedConsoleOutput(trimmed);
        }
    });
}

// Add formatted console output with color coding
function addFormattedConsoleOutput(line) {
    if (!consoleOutput) return;

    const outputDiv = document.createElement('div');
    outputDiv.style.marginBottom = '2px';
    outputDiv.style.fontFamily = 'Courier New, monospace';
    outputDiv.style.fontSize = '0.85em';

    // Color code based on line prefix
    if (line.startsWith('>>')) {
        outputDiv.style.color = '#00bcd4'; // Cyan for major operations
        outputDiv.style.fontWeight = 'bold';
    } else if (line.startsWith('++')) {
        outputDiv.style.color = '#4caf50'; // Green for variable storage
    } else if (line.startsWith('--')) {
        outputDiv.style.color = '#ffc107'; // Yellow for resolution/search
    } else if (line.startsWith('!!')) {
        outputDiv.style.color = '#ff9800'; // Orange for warnings
    } else if (line.startsWith('==')) {
        outputDiv.style.color = '#8bc34a'; // Light green for solutions
        outputDiv.style.fontWeight = 'bold';
    } else if (line.startsWith('**')) {
        outputDiv.style.color = '#9c27b0'; // Purple for cached solutions
    } else if (line.includes('RESULT:') || line.includes('Final result:')) {
        outputDiv.style.color = '#00ff00'; // Bright green for final results
        outputDiv.style.fontWeight = 'bold';
        outputDiv.style.fontSize = '1em';
    } else if (line.includes('Searching for:')) {
        outputDiv.style.color = '#64b5f6'; // Light blue for search queries
        outputDiv.style.fontWeight = 'bold';
    } else if (line.startsWith('   -') || line.startsWith('      ') || line.startsWith('   +')) {
        outputDiv.style.color = '#b0b0b0'; // Gray for sub-items
        outputDiv.style.paddingLeft = '10px';
    } else {
        outputDiv.style.color = '#e0e0e0'; // White/light gray for default
    }

    outputDiv.textContent = line;
    consoleOutput.appendChild(outputDiv);
    consoleOutput.scrollTop = consoleOutput.scrollHeight;
}

// Execute .slut file in loop until solved
async function runUntilSolved() {
    if (!currentFile) {
        addStep('No file loaded', 'error');
        return;
    }

    setRunningState(true);
    addStep(`Looping until solved (max 50 attempts): ${currentFile}`, 'info');
    clearSteps();

    try {
        const result = await invoke('run_until_solved', {
            filePath: currentFile,
            maxAttempts: 50
        });

        addStep('Loop complete: ' + result, 'success');
        await updateStats();

    } catch (error) {
        addStep('Loop error: ' + error, 'error');
        console.error('Loop error:', error);
    } finally {
        setRunningState(false);
    }
}

// Stop execution
async function stopExecution() {
    try {
        await invoke('stop_execution');
        addStep('Execution stopped by user', 'info');
        setRunningState(false);
    } catch (error) {
        addStep('Error stopping execution: ' + error, 'error');
    }
}

// Simulate execution with node highlighting
function simulateExecution() {
    if (!window.globeEngine || currentBlocks.length === 0) return;

    currentBlocks.forEach((block, index) => {
        setTimeout(() => {
            window.globeEngine.highlightNode(index);
            addStep(`Executing block ${index + 1}: ${block.type}`, 'info');
        }, index * 300);
    });
}

// Update stats from backend
async function updateStats() {
    if (!invoke) return;

    try {
        const stats = await invoke('get_cache_stats');

        if (varCount) varCount.textContent = stats.variable_count || 0;
        if (solCount) solCount.textContent = stats.solution_count || 0;
        if (obsCount) obsCount.textContent = stats.observation_count || 0;

        // Try to get accuracy from history
        try {
            const history = await invoke('get_cache_history');
            if (history.attempts && history.attempts.length > 0) {
                const lastAttempt = history.attempts[history.attempts.length - 1];
                updateAccuracyDisplay(lastAttempt.accuracy);
            } else {
                updateAccuracyDisplay(stats.last_accuracy || 0);
            }
        } catch {
            updateAccuracyDisplay(stats.last_accuracy || 0);
        }

    } catch (error) {
        console.error('Failed to update stats:', error);
    }
}

// Update accuracy display
function updateAccuracyDisplay(acc) {
    if (accuracy) accuracy.textContent = acc.toFixed(1) + '%';
    if (accuracyBar) accuracyBar.style.width = acc + '%';

    // Color code accuracy
    if (accuracyBar) {
        if (acc >= 80) {
            accuracyBar.style.background = 'linear-gradient(90deg, #4CAF50 0%, #8BC34A 100%)';
        } else if (acc >= 50) {
            accuracyBar.style.background = 'linear-gradient(90deg, #FFC107 0%, #FFB300 100%)';
        } else {
            accuracyBar.style.background = 'linear-gradient(90deg, #f44336 0%, #d32f2f 100%)';
        }
    }
}

// Add step to execution log
function addStep(message, level = 'info') {
    if (!stepsOutput) return;

    const stepDiv = document.createElement('div');
    stepDiv.className = 'step-line';

    let icon = '•';
    if (level === 'success') icon = '✓';
    else if (level === 'error') icon = '✗';
    else if (level === 'info') icon = 'ℹ';

    stepDiv.innerHTML = `<span class="step-number">${icon}</span> ${message}`;

    // Color code by level
    if (level === 'success') stepDiv.style.borderLeftColor = '#4CAF50';
    else if (level === 'error') stepDiv.style.borderLeftColor = '#f44336';

    stepsOutput.appendChild(stepDiv);
    stepsOutput.scrollTop = stepsOutput.scrollHeight;
}

// Add output to console transcript
function addConsoleOutput(message) {
    if (!consoleOutput) return;

    const outputDiv = document.createElement('div');
    outputDiv.style.marginBottom = '4px';
    outputDiv.textContent = message;

    consoleOutput.appendChild(outputDiv);
    consoleOutput.scrollTop = consoleOutput.scrollHeight;
}

// Clear execution steps
function clearSteps() {
    if (stepsOutput) {
        stepsOutput.innerHTML = '';
    }
    if (consoleOutput) {
        consoleOutput.innerHTML = '';
    }
}

// Set running state
function setRunningState(running) {
    isRunning = running;

    if (statusIndicator) {
        statusIndicator.className = running ?
            'status-indicator status-running' :
            'status-indicator status-idle';
    }

    if (runOnceBtn) runOnceBtn.disabled = running || !currentFile;
    if (runLoopBtn) runLoopBtn.disabled = running || !currentFile;
    if (stopBtn) stopBtn.disabled = !running;
}

// Global functions for button onclick handlers
window.runOnce = runOnce;
window.runUntilSolved = runUntilSolved;
window.stopExecution = stopExecution;
window.resetGlobeView = () => window.globeEngine && window.globeEngine.reset();
window.toggleRotation = () => window.globeEngine && window.globeEngine.toggleRotation();
window.toggleLabels = () => window.globeEngine && window.globeEngine.toggleLabels();

console.log('App.js loaded');
