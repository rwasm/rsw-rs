#!/usr/bin/env node

const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');
const argv = process.argv.slice(2);

if (os.type() === 'Windows_NT') {
  spawnSync(path.join(__dirname, './bin/rsw.exe'), argv, {
    shell: true,
    cwd: process.cwd(),
    encoding: 'utf-8',
    stdio: 'inherit',
  });
} else {
  spawnSync(path.join(__dirname, './bin/rsw'), argv, {
    shell: true,
    cwd: process.cwd(),
    encoding: 'utf-8',
    stdio: 'inherit',
  });
}