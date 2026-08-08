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
      <div class="logo-badge">
        <i class="fa-solid fa-skull skull-icon"></i>
      </div>
      <h1 class="classy-logo">Dayne</h1>
    </div>

    <div class="nav-controls">
      <div class="search-box">
        <i class="fa-solid fa-magnifying-glass search-icon"></i>
        <input 
          class="search-input" 
          type="text" 
          placeholder="Search catalog..." 
          bind:value={searchQuery} 
        />
        {#if searchQuery}
          <button class="clear-search" on:click={() => searchQuery = ''}>
            <i class="fa-solid fa-xmark"></i>
          </button>
        {/if}
      </div>

      <button class="theme-toggle" on:click={toggleTheme} title="Toggle Theme">
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
        <span>All</span>
        <span class="count-badge">{wallpapersList.length}</span>
      </button>
      {#each manifest.categories as cat}
        <button 
          class="chip" 
          class:active={selectedCategory === cat} 
          on:click={() => selectedCategory = cat}
        >
          <span>{cat}</span>
        </button>
      {/each}
    </nav>
  {/if}

  <main class="grid-container">
    {#if loading}
      <div class="status-container">
        <div class="loader-ring"></div>
        <p>Fetching collection...</p>
      </div>
    {:else if errorMsg}
      <div class="error-box">
        <i class="fa-solid fa-triangle-exclamation"></i>
        <div class="error-content">
          <h3>Unable to load catalog</h3>
          <code>{errorMsg}</code>
        </div>
      </div>
    {:else if filteredWallpapers.length === 0}
      <div class="status-container">
        <i class="fa-solid fa-ghost empty-icon"></i>
        <p>No wallpapers match your criteria.</p>
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

  /* Glassmorphic Navbar */
  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.9rem 2rem;
    background: var(--glass-bg);
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border-bottom: 1px solid var(--border-color);
    z-index: 10;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.85rem;
  }

  .logo-badge {
    width: 34px;
    height: 34px;
    border-radius: 10px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: var(--card-shadow);
  }

  .skull-icon {
    font-size: 1.1rem;
    color: var(--accent-color);
    transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }

  .brand:hover .skull-icon {
    transform: rotate(-12deg) scale(1.15);
  }

  .classy-logo {
    font-family: var(--font-brand);
    font-size: 1.4rem;
    font-weight: 700;
    margin: 0;
    letter-spacing: 3px;
    color: var(--text-main);
    text-transform: uppercase;
  }

  .nav-controls {
    display: flex;
    align-items: center;
    gap: 0.85rem;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 0.9rem;
    color: var(--text-sub);
    font-size: 0.85rem;
    pointer-events: none;
  }

  .search-input {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    padding: 0.55rem 2.2rem 0.55rem 2.3rem;
    border-radius: 10px;
    width: 220px;
    outline: none;
    font-size: 0.85rem;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .search-input:focus {
    width: 260px;
    border-color: var(--accent-color);
    box-shadow: 0 0 0 3px rgba(203, 166, 247, 0.15);
  }

  .clear-search {
    position: absolute;
    right: 0.6rem;
    background: transparent;
    border: none;
    color: var(--text-sub);
    cursor: pointer;
    font-size: 0.8rem;
    padding: 0.2rem;
  }

  .theme-toggle {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-main);
    width: 38px;
    height: 38px;
    border-radius: 10px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.95rem;
    transition: all 0.2s ease;
  }

  .theme-toggle:hover {
    background: var(--bg-surface-hover);
    border-color: var(--border-color-hover);
    transform: translateY(-1px);
  }

  /* Sleek Floating Categories Bar */
  .categories {
    display: flex;
    gap: 0.5rem;
    padding: 0.8rem 2rem;
    background: var(--bg-crust);
    border-bottom: 1px solid var(--border-color);
    overflow-x: auto;
    scrollbar-width: none;
  }

  .categories::-webkit-scrollbar {
    display: none;
  }

  .chip {
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    color: var(--text-sub);
    padding: 0.4rem 1rem;
    border-radius: 99px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .chip:hover {
    background: var(--bg-surface-hover);
    color: var(--text-main);
    border-color: var(--border-color-hover);
    transform: translateY(-1px);
  }

  .chip.active {
    background: var(--accent-color);
    color: var(--bg-crust);
    font-weight: 600;
    border-color: transparent;
    box-shadow: 0 4px 12px rgba(203, 166, 247, 0.25);
  }

  .count-badge {
    background: rgba(0, 0, 0, 0.15);
    padding: 0.15rem 0.45rem;
    border-radius: 12px;
    font-size: 0.7rem;
  }

  .grid-container {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 1.5rem;
  }

  .status-container {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 350px;
    color: var(--text-sub);
    gap: 1rem;
  }

  .loader-ring {
    width: 32px;
    height: 32px;
    border: 3px solid var(--border-color);
    border-top-color: var(--accent-color);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .empty-icon {
    font-size: 2rem;
    opacity: 0.5;
  }

  .error-box {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    background: var(--error-bg);
    border: 1px solid var(--error-color);
    color: var(--error-color);
    padding: 1.25rem 1.5rem;
    border-radius: 12px;
    margin: 2rem auto;
    max-width: 600px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
