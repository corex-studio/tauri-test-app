<template>
  <div @click="test">HELLO</div>
  <div>{{ kek }}</div>
  <div>error: {{ error }}</div>
</template>

<script lang="ts" setup>
import { check } from '@tauri-apps/plugin-updater';
import { ref } from 'vue';

const kek = ref<any>(null);
const error = ref<string | null>(null);
const test = async () => {
  try {
    const update = await check({ allowDowngrades: true });
    kek.value = update?.rawJson;
  } catch (e) {
    error.value = String(e as Error);
  }
  // if (update) {
  //   console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`);
  //   let downloaded = 0;
  //   let contentLength = 0;
  //   // alternatively we could also call update.download() and update.install() separately
  //   await update.downloadAndInstall(event => {
  //     switch (event.event) {
  //       case 'Started':
  //         contentLength = event.data.contentLength;
  //         console.log(`started downloading ${event.data.contentLength} bytes`);
  //         break;
  //       case 'Progress':
  //         downloaded += event.data.chunkLength;
  //         console.log(`downloaded ${downloaded} from ${contentLength}`);
  //         break;
  //       case 'Finished':
  //         console.log('download finished');
  //         break;
  //     }
  //   });

  //   console.log('update installed');
  //   await relaunch();
  // }
};
</script>
