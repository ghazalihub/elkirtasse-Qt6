<script lang="ts">
  import { onMount, afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  export let book;
  let pages = $state([]);
  let toc = $state([]);
  let annotations = $state([]);
  let currentPageIndex = $state(0);
  let loading = $state(true);
  let showToc = $state(false);
  let selection = $state('');

  async function loadBook() {
    loading = true;
    try {
      pages = await invoke('get_book_content', { bookId: book.id });
      toc = await invoke('get_book_toc', { bookId: book.id });
      currentPageIndex = 0;
    } catch (e) {
      console.error('Failed to load book content:', e);
    } finally {
      loading = false;
    }
  }

  $effect(() => { if (book) loadBook(); });

  async function loadAnnotations() {
      try {
          annotations = await invoke('get_annotations', { bookId: book.id });
      } catch (e) {
          console.error(e);
      }
  }

  $effect(() => { if (book) loadAnnotations(); });

  function handleMouseUp() {
      const sel = window.getSelection();
      if (sel && sel.toString().trim()) {
          selection = sel.toString().trim();
      } else {
          selection = '';
      }
  }

  async function addAnnotation() {
      if (!selection) return;
      const note = prompt('Add a note for this selection:');
      if (note === null) return;

      const annotation = {
          book_id: book.id,
          page_id: pages[currentPageIndex].id,
          text_selection: selection,
          note: note,
          color: 'yellow'
      };

      try {
          await invoke('save_annotation', { annotation });
          annotations = [...annotations, annotation];
          selection = '';
      } catch (e) {
          console.error(e);
      }
  }

  function nextPage() {
    if (currentPageIndex < pages.length - 1) {
      currentPageIndex++;
    }
  }

  function prevPage() {
    if (currentPageIndex > 0) {
      currentPageIndex--;
    }
  }

  function goToId(id) {
    const index = pages.findIndex(p => p.id === id);
    if (index !== -1) {
      currentPageIndex = index;
      showToc = false;
    }
  }

  let contentElement;
  afterUpdate(() => {
    if (contentElement) contentElement.scrollTop = 0;
  });
</script>

<div class="reader">
  <div class="reader-header">
    <div class="book-meta">
      <h3>{book.title}</h3>
      <button on:click={() => showToc = !showToc} class="toc-btn">
        {showToc ? 'Close Contents' : 'Table of Contents'}
      </button>
    </div>
    <div class="actions">
      {#if selection}
          <button onclick={addAnnotation} class="annotate-btn">Annotate Selection</button>
      {/if}
    </div>
    <div class="pagination">
      <button onclick={prevPage} disabled={currentPageIndex === 0}>Previous</button>
      <span class="page-info">
        {#if pages.length > 0}
            Part {pages[currentPageIndex].part}, Page {pages[currentPageIndex].page}
        {/if}
      </span>
      <button onclick={nextPage} disabled={currentPageIndex === pages.length - 1}>Next</button>
    </div>
  </div>

  <div class="main-view">
    {#if showToc}
      <div class="toc-view">
        {#each toc as entry}
          <button
            class="toc-entry level-{entry.level}"
            on:click={() => goToId(entry.id)}
          >
            {entry.title}
          </button>
        {/each}
      </div>
    {/if}

    <div class="content-area" bind:this={contentElement} onmouseup={handleMouseUp}>
      {#if loading}
        <div class="status">Loading book...</div>
      {:else if pages.length > 0}
        <div class="page-content">
          {@html pages[currentPageIndex].nass.replace(/\n/g, '<br>')}
        </div>

        <div class="annotations-list">
            {#each annotations.filter(a => a.page_id === pages[currentPageIndex].id) as anno}
                <div class="annotation">
                    <strong>"{anno.text_selection}"</strong>: {anno.note}
                </div>
            {/each}
        </div>
      {:else}
        <div class="status">No content available for this book.</div>
      {/if}
    </div>
  </div>
</div>

<style>
  .reader {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: #fff;
  }

  .reader-header {
    padding: 15px 30px;
    border-bottom: 1px solid #eee;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: #fff;
    z-index: 10;
  }

  .book-meta h3 {
    margin: 0 0 5px 0;
    font-size: 1.1rem;
  }

  .toc-btn {
    background: none;
    border: 1px solid #007aff;
    color: #007aff;
    padding: 4px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.85rem;
  }

  .annotate-btn {
      background-color: #ffcc00;
      border: none;
      padding: 5px 15px;
      border-radius: 4px;
      font-weight: 600;
      cursor: pointer;
  }

  .pagination {
    display: flex;
    align-items: center;
    gap: 15px;
  }

  .pagination button {
    padding: 6px 15px;
    border-radius: 6px;
    border: 1px solid #ddd;
    background-color: #fff;
    cursor: pointer;
  }

  .pagination button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .page-info {
    font-size: 0.9rem;
    color: #666;
    min-width: 120px;
    text-align: center;
  }

  .main-view {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .toc-view {
    width: 300px;
    border-right: 1px solid #eee;
    overflow-y: auto;
    background-color: #fafafa;
  }

  .toc-entry {
    width: 100%;
    text-align: left;
    padding: 8px 15px;
    background: none;
    border: none;
    border-bottom: 1px solid #f0f0f0;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .toc-entry:hover {
    background-color: #f0f0f0;
  }

  .level-1 { font-weight: 600; }
  .level-2 { padding-left: 25px; }
  .level-3 { padding-left: 40px; }

  .content-area {
    flex: 1;
    overflow-y: auto;
    padding: 40px 60px;
    line-height: 1.8;
    font-size: 1.2rem;
    color: #2c3e50;
    direction: rtl; /* Arabic books are usually RTL */
  }

  .page-content {
    max-width: 800px;
    margin: 0 auto;
    white-space: pre-wrap;
  }

  .annotations-list {
      margin-top: 40px;
      padding-top: 20px;
      border-top: 2px solid #eee;
      direction: ltr; /* Notes might be in English/mixed */
  }

  .annotation {
      background-color: #fff9c4;
      padding: 10px;
      margin-bottom: 10px;
      border-radius: 4px;
      font-size: 0.95rem;
  }

  .status {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    color: #999;
  }
</style>
