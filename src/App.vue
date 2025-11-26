<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

interface Printer {
  name: string;
  status: string;
}

const printers = ref<Printer[]>([]);
const selectedPrinter = ref<string>('');
const loading = ref(false);
const message = ref('');
const printText = ref('Тестовый текст для печати');
const printHtml = ref('<html><body><h1>Тестовая печать HTML</h1><p>Это тестовый HTML документ для печати.</p></body></html>');

async function loadPrinters() {
  loading.value = true;
  message.value = '';
  try {
    printers.value = await invoke<Printer[]>('get_printers');
    if (printers.value.length > 0 && !selectedPrinter.value) {
      selectedPrinter.value = printers.value[0].name;
    }
  } catch (error) {
    message.value = `Ошибка загрузки принтеров: ${error}`;
  } finally {
    loading.value = false;
  }
}

async function printTextDocument() {
  if (!selectedPrinter.value) {
    message.value = 'Выберите принтер';
    return;
  }

  loading.value = true;
  message.value = '';
  try {
    const result = await invoke<string>('print_text', {
      printerName: selectedPrinter.value,
      text: printText.value,
    });
    message.value = result;
  } catch (error) {
    message.value = `Ошибка печати: ${error}`;
  } finally {
    loading.value = false;
  }
}

async function printHtmlDocument() {
  if (!selectedPrinter.value) {
    message.value = 'Выберите принтер';
    return;
  }

  loading.value = true;
  message.value = '';
  try {
    const result = await invoke<string>('print_html', {
      printerName: selectedPrinter.value,
      html: printHtml.value,
    });
    message.value = result;
  } catch (error) {
    message.value = `Ошибка печати: ${error}`;
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  loadPrinters();
});
</script>

<template>
  <main class="container">
    <h1>Управление принтерами</h1>

    <div class="section">
      <div class="section-header">
        <h2>Список принтеров</h2>
        <button @click="loadPrinters" :disabled="loading" class="refresh-btn">
          {{ loading ? 'Загрузка...' : 'Обновить' }}
        </button>
      </div>

      <div v-if="printers.length === 0 && !loading" class="empty-state">
        Принтеры не найдены
      </div>

      <div v-else class="printers-list">
        <div
          v-for="printer in printers"
          :key="printer.name"
          class="printer-item"
          :class="{ active: selectedPrinter === printer.name }"
          @click="selectedPrinter = printer.name"
        >
          <div class="printer-name">{{ printer.name }}</div>
          <div class="printer-status">{{ printer.status }}</div>
        </div>
      </div>
    </div>

    <div class="section">
      <h2>Печать текста</h2>
      <textarea
        v-model="printText"
        placeholder="Введите текст для печати"
        class="text-input"
        rows="4"
      ></textarea>
      <button @click="printTextDocument" :disabled="loading || !selectedPrinter" class="print-btn">
        Печать текста
      </button>
    </div>

    <div class="section">
      <h2>Печать HTML</h2>
      <textarea
        v-model="printHtml"
        placeholder="Введите HTML для печати"
        class="text-input"
        rows="6"
      ></textarea>
      <button @click="printHtmlDocument" :disabled="loading || !selectedPrinter" class="print-btn">
        Печать HTML
      </button>
    </div>

    <div v-if="message" class="message" :class="{ error: message.includes('Ошибка') }">
      {{ message }}
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 900px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  text-align: center;
  margin-bottom: 30px;
  color: #0f0f0f;
}

h2 {
  margin-top: 0;
  margin-bottom: 15px;
  font-size: 1.2em;
  color: #0f0f0f;
}

.section {
  background-color: #ffffff;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.refresh-btn {
  padding: 8px 16px;
  font-size: 0.9em;
}

.printers-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.printer-item {
  padding: 12px;
  border: 2px solid #e0e0e0;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  background-color: #f9f9f9;
}

.printer-item:hover {
  border-color: #396cd8;
  background-color: #f0f4ff;
}

.printer-item.active {
  border-color: #396cd8;
  background-color: #e8f0ff;
}

.printer-name {
  font-weight: 500;
  margin-bottom: 4px;
  color: #0f0f0f;
}

.printer-status {
  font-size: 0.85em;
  color: #666;
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: #999;
}

.text-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-family: inherit;
  font-size: 0.95em;
  resize: vertical;
  margin-bottom: 15px;
  box-sizing: border-box;
}

.text-input:focus {
  outline: none;
  border-color: #396cd8;
}

.print-btn {
  padding: 10px 20px;
  font-size: 1em;
  background-color: #396cd8;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.print-btn:hover:not(:disabled) {
  background-color: #2952b8;
}

.print-btn:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.message {
  padding: 12px;
  border-radius: 6px;
  margin-top: 20px;
  background-color: #e8f5e9;
  color: #2e7d32;
  text-align: center;
}

.message.error {
  background-color: #ffebee;
  color: #c62828;
}

@media (prefers-color-scheme: dark) {
  h1,
  h2 {
    color: #f6f6f6;
  }

  .section {
    background-color: #1e1e1e;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
  }

  .printer-item {
    background-color: #2a2a2a;
    border-color: #444;
  }

  .printer-item:hover {
    background-color: #333;
    border-color: #396cd8;
  }

  .printer-item.active {
    background-color: #2a3a5a;
    border-color: #396cd8;
  }

  .printer-name {
    color: #f6f6f6;
  }

  .printer-status {
    color: #aaa;
  }

  .text-input {
    background-color: #2a2a2a;
    border-color: #444;
    color: #f6f6f6;
  }

  .text-input:focus {
    border-color: #396cd8;
  }
}
</style>
