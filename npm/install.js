#!/usr/bin/env node

const os = require('os');
const fs = require('fs');
const { install } = require("./binary");
const pkgInfo = require("./package.json");

install();

if (os.type() === 'Windows_NT') {
  pkgInfo.bin = {
    rsw: 'cmd /C ./bin/rsw.exe',
  };
  fs.writeFileSync('./package.json', JSON.stringify(pkgInfo, null, 2));
} else {
  pkgInfo.bin = {
    rsw: './bin/rsw',
  };
  fs.writeFileSync('./package.json', JSON.stringify(pkgInfo, null, 2));
}