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
        <i class="fa-solid fa-image-slash error-icon"></i>
        <span>Preview unavailable</span>
      </div>
    {:else}
      <img 
        src={imageUrl} 
        alt={wallpaper.title} 
        loading="lazy" 
        on:error={() => imageError = true}
      />
      <div class="gradient-overlay"></div>
      <div class="card-action">
        <span class="preview-badge"><i class="fa-solid fa-expand"></i> Inspect</span>
      </div>
    {/if}
  </div>
  <div class="card-info">
    <span class="title" title={wallpaper.title}>{wallpaper.title}</span>
    <span class="category-tag">{wallpaper.category}</span>
  </div>
</div>

<style>
  .card {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 14px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
    box-shadow: var(--card-shadow);
    outline: none;
    position: relative;
  }

  .card:hover, .card:focus-visible {
    transform: translateY(-5px);
    border-color: var(--border-color-hover);
    box-shadow: 0 14px 28px -8px rgba(0, 0, 0, 0.35);
  }

  .image-wrapper {
    width: 100%;
    height: 180px;
    background: var(--bg-crust);
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
    transition: transform 0.5s cubic-bezier(0.25, 1, 0.5, 1);
  }

  .card:hover .image-wrapper img {
    transform: scale(1.06);
  }

  .gradient-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(to top, rgba(17, 17, 27, 0.7) 0%, transparent 60%);
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .card:hover .gradient-overlay {
    opacity: 1;
  }

  .card-action {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .card:hover .card-action {
    opacity: 1;
  }

  .preview-badge {
    background: var(--accent-color);
    color: var(--bg-crust);
    padding: 0.45rem 1.1rem;
    border-radius: 99px;
    font-size: 0.8rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.4rem;
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.3);
    transform: translateY(6px);
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .card:hover .preview-badge {
    transform: translateY(0);
  }

  .error-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-sub);
    font-size: 0.8rem;
  }

  .card-info {
    padding: 0.95rem 1.1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-surface);
  }

  .title {
    font-size: 0.88rem;
    font-weight: 500;
    color: var(--text-main);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 150px;
  }

  .category-tag {
    font-size: 0.7rem;
    color: var(--text-sub);
    background: var(--bg-mantle);
    border: 1px solid var(--border-color);
    padding: 0.2rem 0.55rem;
    border-radius: 6px;
    font-weight: 500;
  }
</style>
