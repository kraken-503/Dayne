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
        <span>Failed to Load</span>
        <small>{imageUrl}</small>
      </div>
    {:else}
      <img 
        src={imageUrl} 
        alt={wallpaper.title} 
        loading="lazy" 
        on:error={() => imageError = true}
      />
    {/if}
  </div>
  <div class="card-info">
    <span class="title">{wallpaper.title}</span>
    <span class="category">{wallpaper.category}</span>
  </div>
</div>

<style>
  .card {
    background: #18181b;
    border: 1px solid #27272a;
    border-radius: 8px;
    overflow: hidden;
    cursor: pointer;
    transition: transform 0.2s, border-color 0.2s;
    outline: none;
  }
  .card:hover, .card:focus {
    transform: translateY(-2px);
    border-color: #3b82f6;
  }
  .image-wrapper {
    width: 100%;
    height: 160px;
    background: #202024;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .image-wrapper img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  .error-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: #ef4444;
    font-size: 0.75rem;
    padding: 0.5rem;
    text-align: center;
  }
  .error-placeholder small {
    color: #a1a1aa;
    font-size: 0.65rem;
    word-break: break-all;
    margin-top: 0.25rem;
  }
  .card-info {
    padding: 0.75rem 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .title {
    font-size: 0.9rem;
    font-weight: 500;
  }
  .category {
    font-size: 0.75rem;
    color: #a1a1aa;
  }
</style>
