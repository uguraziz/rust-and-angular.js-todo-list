const fs = require('fs');
const path = require('path');

const envFile = path.join(__dirname, '..', '.env');
const outputFile = path.join(__dirname, '..', 'public', 'env-config.js');

let envConfig = {};

if (fs.existsSync(envFile)) {
  const envContent = fs.readFileSync(envFile, 'utf8');
  envContent.split('\n').forEach(line => {
    const trimmed = line.trim();
    if (trimmed && !trimmed.startsWith('#')) {
      const [key, ...valueParts] = trimmed.split('=');
      if (key && valueParts.length > 0) {
        const value = valueParts.join('=').replace(/^["']|["']$/g, '');
        envConfig[key.trim()] = value.trim();
      }
    }
  });
}

Object.keys(process.env).forEach(key => {
  if (key.startsWith('API_URL') || key.startsWith('NG_')) {
    envConfig[key] = process.env[key];
  }
});

const jsContent = `window.NG_ENV = ${JSON.stringify(envConfig, null, 2)};`;

fs.writeFileSync(outputFile, jsContent, 'utf8');
console.log('Environment variables injected:', Object.keys(envConfig));

