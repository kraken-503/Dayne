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
        message = 'Set wallpaper is only available inside the desktop application.';
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
      const response = await fetch(originalUrl);
      if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
      
      const blob = await response.blob();
      const fileName = wallpaper.files.original.split('/').pop() || `${wallpaper.id}.png`;
      const blobUrl = URL.createObjectURL(blob);
      
      const link = document.createElement('a');
      link.href = blobUrl;
      link.download = fileName;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      URL.revokeObjectURL(blobUrl);

      message = 'Download complete!';
      messageType = 'success';
    } catch (err: any) {
      console.error('Download error:', err);
      message = 'Failed to download image.';
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
          {wallpaper.dimensions?.width} × {wallpaper.dimensions?.height} px • {wallpaper.category}
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
          {#if downloading}
            <i class="fa-solid fa-circle-notch fa-spin"></i> Downloading...
          {:else}
            <i class="fa-solid fa-download"></i> Download
          {/if}
        </button>

        <button 
          class="btn apply-btn" 
          on:click={handleSetWallpaper} 
          disabled={applying}
        >
          {#if applying}
            <i class="fa-solid fa-circle-notch fa-spin"></i> Applying...
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
    inset: 0;
    background: var(--modal-backdrop);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    padding: 2rem;
    box-sizing: border-box;
    animation: fadeIn 0.2s ease-out;
  }

  .modal {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 20px;
    width: 100%;
    max-width: 850px;
    max-height: 85vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    position: relative;
    box-shadow: 0 24px 48px -12px rgba(0, 0, 0, 0.5);
    animation: popUp 0.25s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .close-btn {
    position: absolute;
    top: 1.2rem;
    right: 1.2rem;
    background: var(--glass-bg);
    backdrop-filter: blur(8px);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    width: 36px;
    height: 36px;
    border-radius: 50%;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.95rem;
    z-index: 10;
    transition: all 0.2s ease;
  }

  .close-btn:hover {
    background: var(--error-color);
    color: #fff;
    border-color: transparent;
    transform: scale(1.05);
  }

  .image-container {
    flex: 1;
    background: var(--bg-crust);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 280px;
    max-height: 55vh;
    overflow: hidden;
    padding: 1rem;
  }

  .image-container img {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: 8px;
  }

  .modal-footer {
    padding: 1.25rem 1.75rem;
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
    font-size: 1.15rem;
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
    padding: 0.4rem 0.85rem;
    border-radius: 8px;
    font-weight: 500;
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
    padding: 0.65rem 1.25rem;
    border-radius: 10px;
    font-weight: 600;
    font-size: 0.85rem;
    cursor: pointer;
    border: none;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.2s ease;
  }

  .download-btn {
    background: var(--bg-surface);
    color: var(--text-main);
    border: 1px solid var(--border-color);
  }

  .download-btn:hover:not(:disabled) {
    background: var(--bg-surface-hover);
    border-color: var(--border-color-hover);
    transform: translateY(-2px);
  }

  .apply-btn {
    background: var(--accent-color);
    color: var(--bg-crust);
  }

  .apply-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 14px rgba(203, 166, 247, 0.3);
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  @keyframes popUp {
    from { transform: scale(0.95); opacity: 0; }
    to { transform: scale(1); opacity: 1; }
  }
</style>
