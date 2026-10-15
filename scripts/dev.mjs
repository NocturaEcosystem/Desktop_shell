import getPort from 'get-port';
import { spawn } from 'child_process';
import fs from 'fs';

async function startDev() {
  const port = await getPort();
  const frontend = process.env.LEPUS_FRONTEND || 'OSD';
  
  console.log(`\n🚀 Starting Tauri with frontend [${frontend}] on random port: ${port}\n`);

  // Explicitly export environment variables for Rust and Vite
  process.env.VITE_PORT = port.toString();
  process.env.LEPUS_FRONTEND = frontend;

  const tempConfigFile = `.tauri-port-${port}.json`;
  fs.writeFileSync(tempConfigFile, JSON.stringify({
    build: {
      devUrl: `http://localhost:${port}`,
      beforeDevCommand: `npm run dev --prefix frontends/${frontend}`
    }
  }));

  const isWindows = process.platform === 'win32';
  
  const tauriProcess = spawn(isWindows ? 'npx.cmd' : 'npx', ['tauri', 'dev', '--config', tempConfigFile], {
    env: process.env,
    stdio: 'inherit',
    shell: isWindows 
  });

  const cleanup = () => {
    try { fs.unlinkSync(tempConfigFile); } catch (e) {}
  };

  tauriProcess.on('close', cleanup);
  process.on('SIGINT', () => {
    cleanup();
    process.exit();
  });
}

startDev();