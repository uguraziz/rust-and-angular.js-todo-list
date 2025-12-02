const fs = require('fs');
const path = require('path');

const outputDir = process.argv[2] || 'dist/browser';
const outputFile = path.join(outputDir, 'env-config.js');

const envConfig = {};

Object.keys(process.env).forEach(key => {
  if (key.startsWith('API_URL') || key.startsWith('NG_') || key === 'API_URL') {
    envConfig[key] = process.env[key];
  }
});

if (process.env.API_URL) {
  envConfig.API_URL = process.env.API_URL;
}

const jsContent = `window.NG_ENV = ${JSON.stringify(envConfig, null, 2)};`;

fs.writeFileSync(outputFile, jsContent, 'utf8');
console.log('Runtime environment variables injected:', Object.keys(envConfig));
console.log('API_URL:', envConfig.API_URL);

