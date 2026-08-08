<script lang="ts">
  import type { Wallpaper, BaseUrls } from '../types';

  export let wallpaper: Wallpaper;
  export let baseUrls: BaseUrls;
  export let onClose: () => void;

  let applying = false;
  let downloading = false;
  let message = '';
  let messageType: 'success' | 'error' = 'success';

  $: previewUrl = `${baseUrls.preview}${wallpaper.files.preview}`;
  $: originalUrl = `${baseUrls.original}${wallpaper.files.original}`;

  async function handleSetWallpaper() {
    applying = true;
    message = '';
    try {
      if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
        const { invoke } = await import('@tauri-apps/api/core');
        await invoke('apply_wallpaper', { filePath: originalUrl });
        message = 'Wallpaper applied successfully!';
        messageType = 'success';
      } else {
        message = 'Set wallpaper function is only available inside Tauri desktop window.';
        messageType = 'error';
      }
    } catch (err: any) {
      console.error('Failed to set wallpaper:', err);
      message = typeof err === 'string' ? err : 'Failed to set wallpaper.';
      messageType = 'error';
    } finally {
      applying = false;
    }
  }

  async function handleDownload() {
    downloading = true;
    message = '';
    try {
      const a = document.createElement('a');
      a.href = originalUrl;
      a.download = wallpaper.files.original;
      a.target = '_blank';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);

      message = 'Download started!';
      messageType = 'success';
    } catch (err) {
      message = 'Failed to trigger download.';
      messageType = 'error';
    } finally {
      downloading = false;
    }
  }
</script>

<div class="backdrop" on:click={onClose} role="dialog" tabindex="-1">
  <div class="modal" on:click|stopPropagation>
    <button class="close-btn" on:click={onClose} title="Close Preview">
      <i class="fa-solid fa-xmark"></i>
    </button>

    <div class="image-container">
      <img src={previewUrl} alt={wallpaper.title} />
    </div>

    <div class="modal-footer">
      <div class="meta">
        <h2>{wallpaper.title}</h2>
        <span class="details">
          {wallpaper.dimensions?.width}x{wallpaper.dimensions?.height} • {wallpaper.category}
        </span>
      </div>

      {#if message}
        <div class="status-msg {messageType}">
          {message}
        </div>
      {/if}

      <div class="actions">
        <button 
          class="btn download-btn" 
          on:click={handleDownload} 
          disabled={downloading}
        >
          <i class="fa-solid fa-download"></i>
          {downloading ? 'Downloading...' : 'Download'}
        </button>

        <button 
          class="btn apply-btn" 
          on:click={handleSetWallpaper} 
          disabled={applying}
        >
          {#if applying}
            <i class="fa-solid fa-spinner fa-spin"></i> Applying...
          {:else}
            <i class="fa-solid fa-desktop"></i> Set Wallpaper
          {/if}
        </button>
      </div>
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: var(--modal-backdrop);
    backdrop-filter: blur(8px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    padding: 1.5rem;
    box-sizing: border-box;
  }

  .modal {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 16px;
    width: 100%;
    max-width: 800px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    box-shadow: 0 20px 50px rgba(0, 0, 0, 0.5);
  }

  .close-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: var(--bg-mantle);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    width: 36px;
    height: 36px;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    z-index: 10;
    transition: background 0.2s;
  }

  .close-btn:hover {
    background: var(--error-color);
    color: #fff;
    border-color: transparent;
  }

  .image-container {
    flex: 1;
    background: var(--bg-crust);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 250px;
    max-height: 55vh;
    overflow: hidden;
  }

  .image-container img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
  }

  .modal-footer {
    padding: 1.25rem 1.5rem;
    background: var(--bg-mantle);
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-top: 1px solid var(--border-color);
    gap: 1rem;
    flex-wrap: wrap;
  }

  .meta h2 {
    margin: 0;
    font-size: 1.1rem;
    font-family: var(--font-brand);
    letter-spacing: 1px;
    color: var(--text-main);
  }

  .details {
    font-size: 0.75rem;
    color: var(--text-sub);
  }

  .status-msg {
    font-size: 0.8rem;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
  }

  .status-msg.success {
    background: rgba(166, 227, 161, 0.15);
    color: #a6e3a1;
    border: 1px solid #a6e3a1;
  }

  .status-msg.error {
    background: var(--error-bg);
    color: var(--error-color);
    border: 1px solid var(--error-color);
  }

  .actions {
    display: flex;
    gap: 0.75rem;
  }

  .btn {
    padding: 0.6rem 1.2rem;
    border-radius: 8px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    border: none;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: transform 0.2s;
  }

  .download-btn {
    background: var(--bg-surface);
    color: var(--text-main);
    border: 1px solid var(--border-color);
  }

  .download-btn:hover {
    background: var(--bg-surface-hover);
    transform: translateY(-2px);
  }

  .apply-btn {
    background: var(--accent-color);
    color: var(--bg-crust);
  }

  .apply-btn:hover {
    transform: translateY(-2px);
  }
</style>
