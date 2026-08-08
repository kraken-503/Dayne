<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Wallpaper, BaseUrls } from '../types';

  export let wallpaper: Wallpaper;
  export let baseUrls: BaseUrls;
  export let onClose: () => void;

  let statusMsg = '';

  $: previewUrl = `${baseUrls.preview}${wallpaper.files.preview}`;
  $: originalUrl = `${baseUrls.original}${wallpaper.files.original}`;

  async function handleSetWallpaper() {
    statusMsg = 'Applying wallpaper...';
    try {
      await invoke('apply_wallpaper', { filePath: originalUrl });
      statusMsg = 'Wallpaper updated!';
    } catch (err: any) {
      statusMsg = `Notice: ${err}`;
    }
  }
</script>

<div class="backdrop" on:click={onClose} on:keydown={(e) => e.key === 'Escape' && onClose()} role="button" tabindex="0">
  <div class="modal" on:click|stopPropagation role="document">
    <button class="close-btn" on:click={onClose}>&times;</button>

    <div class="preview-frame">
      <img src={previewUrl} alt={wallpaper.title} />
    </div>

    <div class="details">
      <div>
        <h2>{wallpaper.title}</h2>
        <p class="meta">{wallpaper.category} • {wallpaper.dimensions.width}x{wallpaper.dimensions.height}</p>
      </div>

      <div class="actions">
        <button class="btn btn-primary" on:click={handleSetWallpaper}>Set Wallpaper</button>
      </div>
    </div>

    {#if statusMsg}
      <p class="status">{statusMsg}</p>
    {/if}
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0, 0, 0, 0.85);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .modal {
    background: var(--bg-surface);
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    width: 90%;
    max-width: 800px;
    padding: 1.5rem;
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .close-btn {
    position: absolute;
    top: 1rem; right: 1rem;
    background: none; border: none;
    color: var(--text-muted);
    font-size: 1.5rem; cursor: pointer;
  }
  .preview-frame {
    width: 100%;
    height: 400px;
    border-radius: 8px;
    overflow: hidden;
    background: #000;
  }
  .preview-frame img {
    width: 100%; height: 100%;
    object-fit: contain;
  }
  .details {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .meta {
    font-size: 0.85rem;
    color: var(--text-muted);
  }
  .btn {
    padding: 0.6rem 1.2rem;
    border-radius: 6px;
    border: none;
    font-weight: 500;
    cursor: pointer;
  }
  .btn-primary {
    background: var(--accent);
    color: white;
  }
  .status {
    font-size: 0.85rem;
    color: var(--text-muted);
  }
</style>
