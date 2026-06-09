const fs = require('fs');
const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

function run(config) {
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'culture-raspimage-convert-'));
  const configPath = path.join(tempDir, 'config.json');
  fs.writeFileSync(configPath, JSON.stringify(config), 'utf8');

  const binaryName = process.platform === 'win32'
    ? 'culture-raspimage-convert.exe'
    : 'culture-raspimage-convert';
  const binaryPath = path.join(__dirname, 'target', 'release', binaryName);
  const result = spawnSync(binaryPath, [configPath], { stdio: 'inherit' });

  if (result.status !== 0) {
    throw new Error(`rust binary exited with code ${result.status}`);
  }
}

module.exports = { run };
