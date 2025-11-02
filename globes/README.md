# OOP Sphere Visualizer Project

A 3D spherical code visualization tool to teach Object-Oriented Programming concepts through spatial/visual learning.

## Project Files

### Standalone HTML Prototypes (Easy to Run)
- `globe-editor-v2.html` - Main interactive Python editor with 3D sphere visualization
- `globe-editor-v2.js` - JavaScript for the globe editor
- `test-widget-standalone.html` - Simplified test version of the visualizer

**To run these**: Just open the HTML files in any modern web browser. No installation needed!

### Theia IDE Extension Files (Advanced)
- `package.json` - Project configuration and dependencies
- `oop-visualizer-widget.js` - Main widget component
- `oop-visualizer-contribution.js` - Command and menu contribution
- `oop-visualizer-frontend-module.js` - Frontend module registration
- `copy-js-simple.js` - Build script for copying files

## Quick Start

### Option 1: Use the Standalone Version (Easiest)
1. Download `globe-editor-v2.html` and `globe-editor-v2.js` to the same folder
2. Open `globe-editor-v2.html` in your web browser
3. Start coding Python in the editor!

### Option 2: Build the Theia Extension
1. Create this folder structure:
   ```
   your-project/
   ├── package.json
   ├── src/
   │   └── browser/
   │       ├── oop-visualizer-widget.js
   │       ├── oop-visualizer-contribution.js
   │       └── oop-visualizer-frontend-module.js
   └── scripts/
       └── copy-js-simple.js
   ```
2. Run `npm install`
3. Run `npm run build`
4. Integrate with Theia IDE workspace

## Features

- **3D Sphere Visualization**: Code blocks arranged on a spherical surface
- **Interactive Nodes**: Click to expand, drag to move, right-click to disable
- **Python Execution**: Run Python code with Skulpt interpreter
- **Data Flow Visualization**: See how data moves between functions
- **Output Rings**: Visual representation of print statements
- **OOP Mapping**:
  - Functions → Yellow nodes
  - Loops → Red nodes
  - Conditions → Purple nodes
  - Classes → Blue nodes

## OOP Pillars

1. **Encapsulation** → Nodes (classes/objects as spheres)
2. **Inheritance** → Connecting lines (parent-child relationships)
3. **Polymorphism** → Node colors/shapes (interface implementations)
4. **Abstraction** → Node clustering/grouping (complexity hiding)

## Controls

- **Left Click**: Expand/collapse code view
- **Right Click**: Enable/disable node
- **Drag**: Move node around sphere
- **Mouse Wheel**: Zoom in/out
- **Arrow Keys**: Navigate camera

## Technologies

- Three.js (3D visualization)
- Skulpt (Python interpreter)
- Theia Extension Framework
- TypeScript/JavaScript

## Status

Working prototype with interactive 3D visualization and Python execution. Ready for Theia extension development.
