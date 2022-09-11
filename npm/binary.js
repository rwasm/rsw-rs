const { Binary } = require('binary-install');
const os = require('os');

const name = 'rsw';
const { rsw_version: version, repository } = require('./package.json');

const supportedPlatforms = [
  {
    TYPE: 'Windows_NT',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-pc-windows-msvc',
    BINARY_NAME: `${name}.exe`
  },
  {
    TYPE: 'Linux',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-unknown-linux-musl',
    BINARY_NAME: name
  },
  {
    TYPE: 'Darwin',
    ARCHITECTURE: 'x64',
    RUST_TARGET: 'x86_64-apple-darwin',
    BINARY_NAME: name
  }
];

const getPlatformMetadata = () => {
  const type = os.type();
  const architecture = os.arch();

  for (let supportedPlatform of supportedPlatforms) {
    if (
      type === supportedPlatform.TYPE &&
      architecture === supportedPlatform.ARCHITECTURE
    ) {
      return supportedPlatform;
    }
  }

  error(
    `Platform with type '${type}' and architecture '${architecture}' is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
      supportedPlatforms
    )}`
  );
};

const getBinary = () => {
  const platformMetadata = getPlatformMetadata();
  // the url for this binary is constructed from values in `package.json`
  const url = `${repository.url}/releases/download/v${version}/rsw-v${version}-${platformMetadata.RUST_TARGET}.tar.gz`;
  return new Binary(platformMetadata.BINARY_NAME, url);
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
