<!-- /admin-dashboard/src/components/PageList.svelte -->
<script>
  export let pages = [];
  export let openEditor;
  export let handleCreatePage;
  export let showCreateModal = false;
  export let newTitle = '';
  export let newSlug = '';
  export let createError = '';
  export let editorError = '';
</script>

<div class="header">
  <h1 style="margin:0;">Web Pages</h1>
  <button class="btn" on:click={() => showCreateModal = true}>+ Create New Page</button>
</div>

{#if showCreateModal}
  <div class="card" style="border-left: 4px solid #4f46e5;">
    <h3 style="margin-top:0;">Create New Page</h3>
    {#if createError}
      <p class="error-msg">{createError}</p>
    {/if}
    <div class="form-group" style="display: flex; gap: 16px;">
      <div style="flex: 1;">
        <label for="new-title">Page Title</label>
        <input type="text" id="new-title" bind:value={newTitle} placeholder="e.g. Services" />
      </div>
      <div style="flex: 1;">
        <label for="new-slug">URL Slug</label>
        <input type="text" id="new-slug" bind:value={newSlug} placeholder="e.g. services" />
      </div>
    </div>
    <div style="display: flex; gap: 10px;">
      <button class="btn" on:click={handleCreatePage}>Create Page</button>
      <button class="btn btn-secondary" on:click={() => showCreateModal = false}>Cancel</button>
    </div>
  </div>
{/if}

{#if editorError}
  <div class="error-msg" style="margin-bottom: 20px;">
    {editorError}
  </div>
{/if}

<div class="card">
  {#if pages.length === 0}
    <p>No pages registered yet. Click "+ Create New Page" to start.</p>
  {:else}
    <table style="width: 100%; border-collapse: collapse;">
      <thead>
        <tr style="border-bottom: 2px solid #e5e7eb; text-align: left;">
          <th style="padding: 12px;">Title</th>
          <th style="padding: 12px;">Slug</th>
          <th style="padding: 12px;">Status</th>
          <th style="padding: 12px; text-align: right;">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each pages as page}
          <tr style="border-bottom: 1px solid #e5e7eb;">
            <td style="padding: 12px; font-weight: 600;">{page.title}</td>
            <td style="padding: 12px; color: #4b5563;">/{page.slug}</td>
            <td style="padding: 12px;">
              <span style="background: {page.status === 'published' ? '#d1fae5' : '#fee2e2'}; color: {page.status === 'published' ? '#065f46' : '#991b1b'}; padding: 4px 10px; border-radius: 12px; font-size: 12px; font-weight: bold;">
                {page.status}
              </span>
            </td>
            <td style="padding: 12px; text-align: right;">
              <button class="btn" on:click={() => openEditor(page)}>Edit Layout</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .card {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    padding: 24px;
    margin-bottom: 20px;
  }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 15px;
    margin-bottom: 30px;
  }
  .btn {
    background: #4f46e5;
    color: #ffffff;
    border: none;
    padding: 10px 18px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
    transition: background 0.2s;
  }
  .btn:hover { background: #4338ca; }
  .btn-secondary { background: #4b5563; }
  .btn-secondary:hover { background: #374151; }
  .form-group {
    margin-bottom: 16px;
  }
  .form-group label {
    display: block;
    font-weight: 600;
    margin-bottom: 6px;
  }
  .form-group input {
    width: 100%;
    padding: 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    box-sizing: border-box;
  }
  .error-msg {
    color: #ef4444; 
    font-weight: bold; 
    background-color: #fee2e2; 
    padding: 12px; 
    border-radius: 6px;
  }
</style>