<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  export let onSelectBook;
  let query = '';
  let results = [];
  let searching = false;
  let error = '';

  async function handleSearch() {
    if (!query.trim()) return;

    searching = true;
    error = '';
    try {
      results = await invoke('search_books', { query });
    } catch (e) {
      error = e.toString();
      console.error('Search failed:', e);
    } finally {
      searching = false;
    }
  }

  function selectResult(result) {
      // Create a book object that the Reader component expects
      onSelectBook({
          id: result.book_id,
          title: result.book_title,
          author: result.author
      });
      // Note: In a full implementation, we'd also pass the page_id to the reader
  }
</script>

<div class="search-container">
  <div class="search-box">
    <input
      type="text"
      bind:value={query}
      placeholder="Search for text in all books..."
      on:keydown={e => e.key === 'Enter' && handleSearch()}
    />
    <button on:click={handleSearch} disabled={searching}>
      {searching ? 'Searching...' : 'Search'}
    </button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="results">
    {#if results.length > 0}
      <p class="summary">Found {results.length} results</p>
      {#each results as result}
        <div class="result-card" on:click={() => selectResult(result)}>
          <div class="result-header">
            <span class="book-title">{result.book_title}</span>
            <span class="location">Part {result.part}, Page {result.page}</span>
          </div>
          <div class="snippet" dir="rtl">
            {result.snippet}
          </div>
        </div>
      {/each}
    {:else if !searching && query}
      <p class="no-results">No results found for "{query}"</p>
    {/if}
  </div>
</div>

<style>
  .search-container {
    padding: 30px;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-box {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
  }

  input {
    flex: 1;
    padding: 12px 15px;
    border: 1px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
    outline: none;
  }

  input:focus {
    border-color: #007aff;
  }

  button {
    padding: 0 25px;
    background-color: #007aff;
    color: white;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    font-weight: 600;
  }

  .results {
    flex: 1;
    overflow-y: auto;
  }

  .summary {
      font-size: 0.9rem;
      color: #666;
      margin-bottom: 15px;
  }

  .result-card {
    background-color: white;
    border: 1px solid #eee;
    border-radius: 10px;
    padding: 15px;
    margin-bottom: 15px;
    cursor: pointer;
    transition: transform 0.1s, box-shadow 0.1s;
  }

  .result-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0,0,0,0.05);
    border-color: #007aff;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .book-title {
    font-weight: 600;
    color: #333;
  }

  .location {
    font-size: 0.8rem;
    color: #888;
  }

  .snippet {
    font-size: 0.95rem;
    line-height: 1.5;
    color: #444;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .error {
    color: #d32f2f;
    margin-bottom: 10px;
    padding: 10px;
    background-color: #ffebee;
    border-radius: 4px;
  }

  .no-results {
      text-align: center;
      margin-top: 50px;
      color: #999;
  }
</style>
