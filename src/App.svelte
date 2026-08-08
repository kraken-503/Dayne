<script lang="ts">
  import { onMount } from 'svelte';
  import type { Wallpaper, CatalogManifest } from './lib/types';
  import WallpaperCard from './lib/components/WallpaperCard.svelte';
  import PreviewModal from './lib/components/PreviewModal.svelte';

  let manifest: CatalogManifest | null = null;
  let selectedCategory: string = 'All';
  let searchQuery: string = '';
  let activeWallpaper: Wallpaper | null = null;
  let loading: boolean = true;
  let errorMsg: string | null = null;
  let theme: 'dark' | 'light' = 'dark';

  $: wallpapersList = manifest?.wallpapers ?? [];

  $: filteredWallpapers = wallpapersList.filter(item => {
    const matchesCategory = selectedCategory === 'All' || 
      (item.category && item.category.trim().toLowerCase() === selectedCategory.trim().toLowerCase());
      
    const query = searchQuery.trim().toLowerCase();
    const matchesQuery = !query ||
      (item.title && item.title.toLowerCase().includes(query)) ||
      (item.tags && item.tags.some(t => t.toLowerCase().includes(query)));

    return matchesCategory && matchesQuery;
  });

  function toggleTheme() {
    theme = theme === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', theme);
  }

  onMount(async () => {
    document.documentElement.setAttribute('data-theme', theme);
    try {
      if (typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window) {
        const { invoke } = await import('@tauri-apps/api/core');
        manifest = await invoke<CatalogManifest>('fetch_catalog');
      } else {
        errorMsg = "Browser preview mode active. Please launch Dayne using 'npm run tauri dev'.";
      }
    } catch (err: any) {
      console.error('Failed to fetch catalog:', err);
      errorMsg = typeof err === 'string' ? err : JSON.stringify(err);
    } finally {
      loading = false;
    }
  });
</script>

<div class="shell">
  <header class="navbar">
    <div class="brand">
      <i class="fa-solid fa-skull skull-icon"></i>
      <h1 class="classy-logo">Dayne</h1>
    </div>

    <div class="nav-controls">
      <div class="search-box">
        <i class="fa-solid fa-magnifying-glass search-icon"></i>
        <input 
          class="search-input" 
          type="text" 
          placeholder="Search wallpapers..." 
          bind:value={searchQuery} 
        />
      </div>

      <button class="theme-toggle" on:click={toggleTheme} title="Toggle Dark/Light Mode">
        {#if theme === 'dark'}
          <i class="fa-solid fa-sun"></i>
        {:else}
          <i class="fa-solid fa-moon"></i>
        {/if}
      </button>
    </div>
  </header>

  {#if manifest && manifest.categories}
    <nav class="categories">
      <button 
        class="chip" 
        class:active={selectedCategory === 'All'} 
        on:click={() => selectedCategory = 'All'}
      >
        All ({wallpapersList.length})
      </button>
      {#each manifest.categories as cat}
        <button 
          class="chip" 
          class:active={selectedCategory === cat} 
          on:click={() => selectedCategory = cat}
        >
          {cat}
        </button>
      {/each}
    </nav>
  {/if}

  <main class="grid-container">
    {#if loading}
      <div class="status-container">
        <i class="fa-solid fa-circle-notch fa-spin spinner"></i>
        <p>Loading wallpapers...</p>
      </div>
    {:else if errorMsg}
      <div class="error-box">
        <h3>Catalog Error</h3>
        <code>{errorMsg}</code>
      </div>
    {:else if filteredWallpapers.length === 0}
      <div class="status-container">
        <p>No wallpapers found in catalog.</p>
      </div>
    {:else}
      <div class="grid">
        {#each filteredWallpapers as wallpaper (wallpaper.id + wallpaper.files.original)}
          <WallpaperCard
            {wallpaper}
            baseUrl={manifest?.baseUrls?.thumbnail ?? ''}
            onSelect={() => activeWallpaper = wallpaper}
          />
        {/each}
      </div>
    {/if}
  </main>

  {#if activeWallpaper && manifest}
    <PreviewModal
      wallpaper={activeWallpaper}
      baseUrls={manifest.baseUrls}
      onClose={() => activeWallpaper = null}
    />
  {/if}
</div>

<style>
  .shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    background-color: var(--bg-base);
    color: var(--text-main);
  }

  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.85rem 1.75rem;
    background-color: var(--bg-mantle);
    border-bottom: 1px solid var(--border-color);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .skull-icon {
    font-size: 1.25rem;
    color: var(--accent-color);
    transition: transform 0.3s ease;
  }

  .brand:hover .skull-icon {
    transform: rotate(-10deg) scale(1.1);
  }

  .classy-logo {
    font-family: var(--font-brand);
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    letter-spacing: 2px;
    color: var(--text-main);
    text-transform: uppercase;
  }

  .nav-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 0.85rem;
    color: var(--text-sub);
    font-size: 0.85rem;
  }

  .search-input {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 0.5rem 0.85rem 0.5rem 2.2rem;
    border-radius: 8px;
    width: 220px;
    outline: none;
    font-size: 0.85rem;
    transition: border-color 0.25s ease;
  }

  .search-input:focus {
    border-color: var(--accent-color);
  }

  .theme-toggle {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    width: 36px;
    height: 36px;
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 1rem;
    transition: background 0.2s, border-color 0.2s;
  }

  .theme-toggle:hover {
    background: var(--bg-surface-hover);
    border-color: var(--accent-color);
  }

  .categories {
    display: flex;
    gap: 0.6rem;
    padding: 0.75rem 1.75rem;
    background-color: var(--bg-crust);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
  }

  .chip {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-sub);
    padding: 0.35rem 0.9rem;
    border-radius: 20px;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
    transition: all 0.2s ease;
  }

  .chip:hover {
    background: var(--bg-surface-hover);
    color: var(--text-main);
  }

  .chip.active {
    background: var(--accent-color);
    color: var(--bg-crust);
    font-weight: 600;
    border-color: transparent;
  }

  .grid-container {
    flex: 1;
    overflow-y: auto;
    padding: 1.75rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.35rem;
  }

  .status-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 300px;
    color: var(--text-sub);
    gap: 1rem;
  }

  .spinner {
    font-size: 1.8rem;
    color: var(--accent-color);
  }

  .error-box {
    background: var(--error-bg);
    border: 1px solid var(--error-color);
    color: var(--error-color);
    padding: 1.25rem;
    border-radius: 8px;
    margin: 2rem auto;
    max-width: 600px;
  }
</style>
