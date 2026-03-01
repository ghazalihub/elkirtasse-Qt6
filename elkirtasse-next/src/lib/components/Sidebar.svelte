<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';

  export let library = $state([]);
  export let onSelectBook;

  let expandedGroups = $state(new Set());
  let importing = $state(false);

  async function importBook() {
      const selected = await open({
          multiple: false,
          filters: [{
              name: 'Maktaba Shamela',
              extensions: ['mdb', 'bok']
          }]
      });

      if (selected && !Array.isArray(selected)) {
          importing = true;
          try {
              await invoke('import_book_mdb_command', { path: selected });
              alert('Book imported and indexed successfully!');
              // In a real app we would refresh the library list here
          } catch (e) {
              alert('Import failed: ' + e);
          } finally {
              importing = false;
          }
      }
  }

  function toggleGroup(id) {
    if (expandedGroups.has(id)) {
      expandedGroups.delete(id);
    } else {
      expandedGroups.add(id);
    }
    expandedGroups = expandedGroups;
  }
</script>

<div class="sidebar">
  <div class="header">
    <h2>Library</h2>
    <button onclick={importBook} disabled={importing} class="import-btn">
        {importing ? 'Importing...' : 'Import MDB/BOK'}
    </button>
  </div>
  <div class="groups">
    {#each library as group}
      <div class="group">
        <button class="group-header" on:click={() => toggleGroup(group.id)}>
          <span class="icon">{expandedGroups.has(group.id) ? '▾' : '▸'}</span>
          {group.name}
        </button>
        {#if expandedGroups.has(group.id)}
          <div class="book-list">
            {#each group.books as book}
              <button class="book-item" on:click={() => onSelectBook(book)}>
                <span class="book-icon">📖</span>
                <div class="book-info">
                    <span class="book-title">{book.title}</span>
                    <span class="book-author">{book.author}</span>
                </div>
              </button>
            {/each}
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .sidebar {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .header {
    padding: 20px;
    border-bottom: 1px solid #eee;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .import-btn {
      background-color: #28a745;
      color: white;
      border: none;
      padding: 8px;
      border-radius: 4px;
      font-weight: 600;
      cursor: pointer;
  }

  .import-btn:disabled {
      background-color: #94d3a2;
      cursor: not-allowed;
  }

  .groups {
    padding: 10px 0;
  }

  .group-header {
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    padding: 10px 20px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    display: flex;
    align-items: center;
    color: #444;
  }

  .group-header:hover {
    background-color: #f0f0f0;
  }

  .icon {
    margin-right: 10px;
    width: 15px;
  }

  .book-list {
    background-color: #fafafa;
  }

  .book-item {
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    padding: 8px 20px 8px 40px;
    cursor: pointer;
    display: flex;
    align-items: flex-start;
    transition: background-color 0.1s;
  }

  .book-item:hover {
    background-color: #eef6ff;
  }

  .book-icon {
    margin-right: 12px;
    margin-top: 2px;
  }

  .book-info {
      display: flex;
      flex-direction: column;
  }

  .book-title {
    font-size: 0.9rem;
    font-weight: 500;
    color: #333;
  }

  .book-author {
    font-size: 0.75rem;
    color: #777;
  }
</style>
