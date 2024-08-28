const { invoke } = window.__TAURI__.tauri;

async function getHomeDir() {
  try {
    const homeDir = await invoke('get_home_dir');
    console.log('Home directory:', homeDir);
    return homeDir;
  } catch (error) {
    console.error('Failed to get home directory:', error);
  }
}

const homeDir = getHomeDir()
const userPath = `{homeDir}/FoxGDPS/FoxGDPS.exe`;

document.getElementById("run").addEventListener("click", async () => {
  document.getElementById("status-message").innerText = "Running...";
  document.getElementById("status-message").style = "yellow";
  try {
      await invoke('run', { userPath });
      // clear any error message if execution succeeds
      document.getElementById("status-message").innerText = "FoxGDPS ran successfully, you might need to wait a few seconds for it to launch";
      document.getElementById("status-message").style.color = "lightgreen";
  } catch (error) {
      // display the error message in red
      document.getElementById("status-message").innerText = error;
      document.getElementById("status-message").style.color = "red";
  }
});
