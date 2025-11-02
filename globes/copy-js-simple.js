const fs = require('fs');
const path = require('path');

function copyFileSync(source, target) {
    let targetFile = target;
    if (fs.existsSync(target)) {
        if (fs.lstatSync(target).isDirectory()) {
            targetFile = path.join(target, path.basename(source));
        }
    }
    fs.writeFileSync(targetFile, fs.readFileSync(source));
}

function copyFolderRecursiveSync(source, target) {
    let files = [];
    const targetFolder = path.join(target, path.basename(source));
    if (!fs.existsSync(targetFolder)) {
        fs.mkdirSync(targetFolder, { recursive: true });
    }
    if (fs.lstatSync(source).isDirectory()) {
        files = fs.readdirSync(source);
        files.forEach(function (file) {
            const curSource = path.join(source, file);
            if (fs.lstatSync(curSource).isDirectory()) {
                copyFolderRecursiveSync(curSource, targetFolder);
            } else {
                copyFileSync(curSource, targetFolder);
            }
        });
    }
}

try {
    // Create lib directory
    if (!fs.existsSync('lib')) {
        fs.mkdirSync('lib', { recursive: true });
    }
    if (!fs.existsSync('lib/browser')) {
        fs.mkdirSync('lib/browser', { recursive: true });
    }
    
    // Copy files from src/browser to lib/browser
    const files = fs.readdirSync('src/browser');
    files.forEach(file => {
        if (file.endsWith('.js')) {
            copyFileSync(path.join('src/browser', file), path.join('lib/browser', file));
        }
    });
    
    console.log('✅ JavaScript files copied successfully!');
} catch (err) {
    console.error('❌ Error copying files:', err);
    process.exit(1);
} 
