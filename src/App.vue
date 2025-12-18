<template>
  <div style="padding: 20px">
    <div style="margin-bottom: 20px">
      <div style="margin-bottom: 10px"><strong>Current Version:</strong> {{ currentVersion || 'Loading...' }}</div>
      <div style="margin-bottom: 10px">
        <button @click="test" style="padding: 10px 20px; margin-right: 10px">Check for Updates</button>
        <button @click="openDevtools" style="padding: 10px 20px; margin-right: 10px">Open DevTools</button>
      </div>
      <div style="margin-bottom: 10px; display: flex; align-items: center; gap: 10px">
        <label>
          <strong>Downgrade to version:</strong>
          <input
            v-model="targetVersion"
            type="text"
            placeholder="e.g., 0.1.0"
            style="margin-left: 8px; padding: 5px 10px; border: 1px solid #ccc; border-radius: 4px"
          />
        </label>
        <button
          @click="downgradeToVersion"
          :disabled="!targetVersion || downloadStatus !== 'idle'"
          style="padding: 5px 15px"
        >
          Install Version
        </button>
      </div>
    </div>

    <div v-if="updateInfo" style="margin-bottom: 20px">
      <h3>Update Info:</h3>
      <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px; overflow: auto">{{ updateInfo }}</pre>
    </div>

    <div v-if="downloadStatus !== 'idle'" style="margin-bottom: 20px">
      <h3>Download Status: {{ downloadStatus }}</h3>
      <div v-if="contentLength > 0" style="margin-top: 10px">
        <div style="width: 100%; background: #e0e0e0; border-radius: 4px; height: 24px; position: relative">
          <div
            :style="{
              width: `${progressPercent}%`,
              background: '#4CAF50',
              height: '100%',
              borderRadius: '4px',
              transition: 'width 0.3s ease',
            }"
          ></div>
        </div>
        <div style="margin-top: 8px; font-size: 14px">
          {{ formatBytes(downloaded) }} / {{ formatBytes(contentLength) }} ({{ progressPercent.toFixed(1) }}%)
        </div>
      </div>
    </div>

    <div v-if="error" style="color: red; margin-top: 20px"><strong>Error:</strong> {{ error }}</div>
  </div>
</template>

<script lang="ts" setup>
import { check } from '@tauri-apps/plugin-updater';
import { invoke } from '@tauri-apps/api/core';
import { relaunch } from '@tauri-apps/plugin-process';
import { ref, computed } from 'vue';

const updateInfo = ref<string | null>(null);
const error = ref<string | null>(null);
const currentVersion = ref<string | null>(null);
const targetVersion = ref<string>('');
const downloadStatus = ref<'idle' | 'checking' | 'downloading' | 'installing' | 'finished'>('idle');
const downloaded = ref<number>(0);
const contentLength = ref<number>(0);

const progressPercent = computed(() => {
  if (contentLength.value === 0) return 0;
  return (downloaded.value / contentLength.value) * 100;
});

const formatBytes = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
};

const loadCurrentVersion = async () => {
  try {
    currentVersion.value = await invoke<string>('get_app_version');
  } catch (e) {
    console.error('Failed to load current version:', e);
  }
};

const openDevtools = async () => {
  try {
    await invoke('open_devtools');
  } catch (e) {
    error.value = `Failed to open devtools: ${String(e as Error)}`;
  }
};

const downgradeToVersion = async () => {
  if (!targetVersion.value.trim()) {
    error.value = 'Please enter a target version';
    return;
  }

  try {
    error.value = null;
    downloadStatus.value = 'checking';
    downloaded.value = 0;
    contentLength.value = 0;

    const update = await check({ allowDowngrades: true });

    if (update) {
      if (update.version !== targetVersion.value.trim()) {
        error.value = `Available version is ${update.version}, but requested ${targetVersion.value}. Please check if the version exists.`;
        downloadStatus.value = 'idle';
        return;
      }

      updateInfo.value = JSON.stringify(update.rawJson, null, 2);
      console.log(`Installing version ${update.version} (downgrade from ${update.currentVersion})`);

      downloadStatus.value = 'downloading';
      let totalDownloaded = 0;
      let totalContentLength = 0;

      await update.downloadAndInstall(event => {
        switch (event.event) {
          case 'Started':
            totalContentLength = event.data.contentLength || 0;
            contentLength.value = totalContentLength;
            downloaded.value = 0;
            console.log(`Started downloading ${event.data.contentLength} bytes`);
            break;
          case 'Progress':
            totalDownloaded += event.data.chunkLength;
            downloaded.value = totalDownloaded;
            console.log(`Downloaded ${totalDownloaded} from ${totalContentLength}`);
            break;
          case 'Finished':
            downloadStatus.value = 'finished';
            console.log('Download finished');
            break;
        }
      });

      console.log('Update installed');
      downloadStatus.value = 'installing';
      await relaunch();
    } else {
      error.value = `Version ${targetVersion.value} not found or no updates available`;
      downloadStatus.value = 'idle';
    }
  } catch (e) {
    error.value = String(e as Error);
    downloadStatus.value = 'idle';
  }
};

const test = async () => {
  try {
    error.value = null;
    downloadStatus.value = 'checking';
    downloaded.value = 0;
    contentLength.value = 0;

    const update = await check({ allowDowngrades: true });

    if (update) {
      updateInfo.value = JSON.stringify(update.rawJson, null, 2);
      console.log(`Found update ${update.version} from ${update.date} with notes ${update.body}`);
      console.log(`Current version: ${update.currentVersion}, Available version: ${update.version}`);

      downloadStatus.value = 'downloading';
      let totalDownloaded = 0;
      let totalContentLength = 0;

      await update.downloadAndInstall(event => {
        switch (event.event) {
          case 'Started':
            totalContentLength = event.data.contentLength || 0;
            contentLength.value = totalContentLength;
            downloaded.value = 0;
            console.log(`Started downloading ${event.data.contentLength} bytes`);
            break;
          case 'Progress':
            totalDownloaded += event.data.chunkLength;
            downloaded.value = totalDownloaded;
            console.log(`Downloaded ${totalDownloaded} from ${totalContentLength}`);
            break;
          case 'Finished':
            downloadStatus.value = 'finished';
            console.log('Download finished');
            break;
        }
      });

      console.log('Update installed');
      downloadStatus.value = 'installing';
      await relaunch();
    } else {
      updateInfo.value = 'No updates available';
      downloadStatus.value = 'idle';
    }
  } catch (e) {
    error.value = String(e as Error);
    downloadStatus.value = 'idle';
  }
};

loadCurrentVersion();
</script>
