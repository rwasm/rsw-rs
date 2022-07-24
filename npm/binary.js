const { Binary } = require('./binary-install');
const os = require('os');

const windows = 'x86_64-pc-windows-msvc';

const { rsw_version: version, repository } = require('./package.json');

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === 'Windows_NT' && arch === 'x64') {
    return windows;
  }
  if (type === 'Linux' && arch === 'x64') {
    return 'x86_64-unknown-linux-musl';
  }
  if (type === 'Darwin' && (arch === 'x64' || arch === 'arm64')) {
    return 'x86_64-apple-darwin';
  }

  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
  const platform = getPlatform();
  // the url for this binary is constructed from values in `package.json`
  const url = `${repository.url}/releases/download/v${version}/rsw-v${version}-${platform}.tar.gz`;
  return new Binary(platform === windows ? 'rsw.exe' : 'rsw', url);
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

module.exports = {
  install,
  run
};
