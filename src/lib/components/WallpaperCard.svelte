<script lang="ts">
  import type { Wallpaper } from '../types';

  export let wallpaper: Wallpaper;
  export let baseUrl: string;
  export let onSelect: () => void;

  let imageError = false;

  $: cleanBase = baseUrl ? (baseUrl.endsWith('/') ? baseUrl : `${baseUrl}/`) : '';
  $: cleanFile = wallpaper?.files?.thumbnail ? (wallpaper.files.thumbnail.startsWith('/') ? wallpaper.files.thumbnail.slice(1) : wallpaper.files.thumbnail) : '';
  $: imageUrl = `${cleanBase}${cleanFile}`;
</script>

<div 
  class="card" 
  on:click={onSelect} 
  on:keydown={(e) => e.key === 'Enter' && onSelect()} 
  role="button" 
  tabindex="0"
>
  <div class="image-wrapper">
    {#if imageError}
      <div class="error-placeholder">
        <i class="fa-solid fa-triangle-exclamation error-icon"></i>
        <span>Image Not Available</span>
      </div>
    {:else}
      <img 
        src={imageUrl} 
        alt={wallpaper.title} 
        loading="lazy" 
        on:error={() => imageError = true}
      />
      <div class="overlay">
        <span class="view-btn"><i class="fa-solid fa-expand"></i> Preview</span>
      </div>
    {/if}
  </div>
  <div class="card-info">
    <span class="title">{wallpaper.title}</span>
    <span class="badge">{wallpaper.category}</span>
  </div>
</div>

<style>
  .card {
    background: var(--ctp-surface0);
    border: 1px solid var(--ctp-surface1);
    border-radius: 12px;
    overflow: hidden;
    cursor: pointer;
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1), 
                box-shadow 0.3s ease, 
                border-color 0.3s ease;
    outline: none;
    position: relative;
  }

  .card:hover, .card:focus {
    transform: translateY(-6px) scale(1.02);
    border-color: var(--ctp-mauve);
    box-shadow: 0 12px 24px -8px rgba(17, 11, 27, 0.6),
                0 0 16px rgba(203, 166, 247, 0.2);
  }

  .image-wrapper {
    width: 100%;
    height: 170px;
    background: var(--ctp-mantle);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
  }

  .image-wrapper img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.4s ease;
  }

  .card:hover .image-wrapper img {
    transform: scale(1.08);
  }

  .overlay {
    position: absolute;
    inset: 0;
    background: rgba(17, 17, 27, 0.55);
    backdrop-filter: blur(2px);
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.25s ease;
  }

  .card:hover .overlay {
    opacity: 1;
  }

  .view-btn {
    background: var(--ctp-mauve);
    color: var(--ctp-crust);
    padding: 0.45rem 1rem;
    border-radius: 20px;
    font-size: 0.8rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    transform: translateY(8px);
    transition: transform 0.25s ease;
  }

  .card:hover .view-btn {
    transform: translateY(0);
  }

  .error-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
    color: var(--ctp-red);
    font-size: 0.8rem;
  }

  .error-icon {
    font-size: 1.2rem;
  }

  .card-info {
    padding: 0.85rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--ctp-surface0);
  }

  .title {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--ctp-text);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 140px;
  }

  .badge {
    font-size: 0.7rem;
    background: var(--ctp-surface1);
    color: var(--ctp-subtext0);
    padding: 0.2rem 0.5rem;
    border-radius: 6px;
  }
</style>
