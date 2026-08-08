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

  $: wallpapersList = manifest?.wallpapers ?? [];

  $: filteredWallpapers = wallpapersList.filter(item => {
    // Case-insensitive category match
    const matchesCategory = selectedCategory === 'All' || 
      (item.category && item.category.trim().toLowerCase() === selectedCategory.trim().toLowerCase());
      
    const query = searchQuery.trim().toLowerCase();
    const matchesQuery = !query ||
      (item.title && item.title.toLowerCase().includes(query)) ||
      (item.tags && item.tags.some(t => t.toLowerCase().includes(query)));

    return matchesCategory && matchesQuery;
  });

  onMount(async () => {
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
      <span class="logo-dot"></span>
      <h1>Dayne</h1>
    </div>
    <input class="search" type="text" placeholder="Search wallpapers..." bind:value={searchQuery} />
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
        <p>Loading wallpapers...</p>
      </div>
    {:else if errorMsg}
      <div class="error-box">
        <h3>Catalog Error</h3>
        <code>{errorMsg}</code>
      </div>
    {:else if filteredWallpapers.length === 0}
      <div class="status-container">
        <p>No wallpapers found under '{selectedCategory}'. Total loaded: {wallpapersList.length}</p>
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
  :global(html, body) {
    margin: 0;
    padding: 0;
    height: 100%;
    width: 100%;
    background-color: #0f0f11;
    color: #f4f4f5;
    overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .shell {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
  }

  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #27272a;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .logo-dot {
    width: 8px;
    height: 8px;
    background: #3b82f6;
    border-radius: 50%;
  }

  .brand h1 {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 0;
  }

  .search {
    background: #18181b;
    border: 1px solid #27272a;
    color: #f4f4f5;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    width: 220px;
    outline: none;
  }

  .categories {
    display: flex;
    gap: 0.5rem;
    padding: 0.75rem 1.5rem;
    border-bottom: 1px solid #27272a;
    overflow-x: auto;
  }

  .chip {
    background: none;
    border: 1px solid #27272a;
    color: #a1a1aa;
    padding: 0.3rem 0.8rem;
    border-radius: 20px;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
  }

  .chip.active {
    background: #18181b;
    color: #f4f4f5;
    border-color: #a1a1aa;
  }

  .grid-container {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
    gap: 1.25rem;
  }

  .status-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 200px;
    color: #a1a1aa;
  }

  .error-box {
    background: #2a1215;
    border: 1px solid #7f1d1d;
    color: #fca5a5;
    padding: 1.25rem;
    border-radius: 8px;
    margin: 2rem auto;
    max-width: 600px;
  }

  .error-box code {
    display: block;
    margin-top: 0.5rem;
    color: #fff;
    word-break: break-all;
  }
</style>
