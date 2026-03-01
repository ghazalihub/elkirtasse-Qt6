<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import Sidebar from '../lib/components/Sidebar.svelte';
  import Reader from '../lib/components/Reader.svelte';
  import Search from '../lib/components/Search.svelte';

  let library = $state([]);
  let selectedBook = $state(null);
  let view = $state('library'); // 'library', 'reader', 'search'

  async function loadLibrary() {
    try {
      library = await invoke('get_library');
    } catch (e) {
      console.error('Failed to load library:', e);
    }
  }

  onMount(loadLibrary);

  function selectBook(book) {
    selectedBook = book;
    view = 'reader';
  }

  function toggleSearch() {
    view = view === 'search' ? 'library' : 'search';
  }
</script>

<main class="container">
  <div class="sidebar-container">
    <Sidebar {library} onSelectBook={selectBook} />
    <button onclick={toggleSearch} class="search-toggle">
        {view === 'search' ? 'Back to Library' : 'Search All Books'}
    </button>
  </div>

  <div class="content">
    {#if view === 'search'}
      <Search onSelectBook={selectBook} />
    {:else if selectedBook}
      <Reader book={selectedBook} />
    {:else}
      <div class="welcome">
        <h1>Welcome to Elkirtasse Next</h1>
        <p>Select a book from the sidebar to start reading.</p>
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    display: flex;
    height: 100vh;
    width: 100vw;
    margin: 0;
    font-family: 'Inter', sans-serif;
    background-color: #f9f9f9;
  }

  .sidebar-container {
    width: 300px;
    background-color: #fff;
    border-right: 1px solid #eee;
    display: flex;
    flex-direction: column;
    box-shadow: 2px 0 5px rgba(0,0,0,0.02);
  }

  .content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .search-toggle {
    margin: 10px;
    padding: 10px;
    background-color: #007aff;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
    transition: background-color 0.2s;
  }

  .search-toggle:hover {
    background-color: #0056b3;
  }

  .welcome {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    color: #888;
  }

  h1 {
      color: #333;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #f9f9f9;
  }
</style>
