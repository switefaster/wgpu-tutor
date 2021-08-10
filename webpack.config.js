const path = require('path');

module.exports = {
    entry: './websrc/index.js',
    output: {
        filename: 'bundle.js',
        path: path.resolve(__dirname)
    },
    performance: {
        maxEntrypointSize: 1024000,
        maxAssetSize: 1024000
    },
    mode: 'production'
}
