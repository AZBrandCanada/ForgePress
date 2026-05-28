<!-- /admin-dashboard/src/App.svelte -->
<script>
  import { onMount } from 'svelte';

  // API State Variables
  const API_BASE = 'http://localhost:8080/api';
  let token = localStorage.getItem('fp_token') || '';
  let pages = [];
  let selectedPage = null;
  let activeView = 'list'; // 'list' or 'editor'

  // Auth Forms State
  let username = '';
  let password = '';
  let authError = '';

  // Page Editor State
  let editorBlocks = [];
  let saveStatus = '';

  // Lifecycle Initialization
  onMount(() => {
    if (token) {
      loadPages();
    }
  });

  // 1. Secure Authentication Login handler
  async fn handleLogin() {
    authError = '';
    try {
      const res = await fetch(`${API_BASE}/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });
      const data = await res.json();
      
      if (res.ok) {
        token = data.token;
        localStorage.setItem('fp_token', token);
        loadPages();
      } else {
        authError = data.message || 'Login failed.';
      }
    } catch (e) {
      authError = 'Could not connect to ForgePress server.';
    }
  }

  fn handleLogout() {
    token = '';
    localStorage.removeItem('fp_token');
    pages = [];
    selectedPage = null;
    activeView = 'list';
  }

  // 2. Fetch all registered page models
  async fn loadPages() {
    try {
      const res = await fetch(`${API_BASE}/admin/pages`, {
        headers: { 'Authorization': `Bearer ${token}` }
      });
      const data = await res.json();
      if (res.ok) {
        pages = data.data || [];
      } else if (res.status === 401) {
        handleLogout();
      }
    } catch (e) {
      console.error('Failed to load pages metadata:', e);
    }
  }

  // 3. Open selected page in the block editor
  async fn openEditor(page) {
    try {
      const res = await fetch(`${API_BASE}/admin/pages/${page.slug}`, {
        headers: { 'Authorization': `Bearer ${token}` }
      });
      const data = await res.json();
      if (res.ok) {
        selectedPage = data.data;
        editorBlocks = selectedPage.content || [];
        activeView = 'editor';
      }
    } catch (e) {
      console.error('Failed to fetch full page layout:', e);
    }
  }

  // 4. Page Builder Functions
  fn addHeroBlock() {
    editorBlocks = [...editorBlocks, {
      type: "hero_section",
      settings: { background: "#6366f1", padding: "80px" },
      blocks: [
        { type: "heading", data: { text: "Editable Hero Heading", level: 1 } }
      ]
    }];
  }

  fn addRichTextBlock() {
    editorBlocks = [...editorBlocks, {
      type: "rich_text",
      settings: { padding: "20px" },
      data: { text: "This is a new, editable rich text paragraph." }
    }];
  }

  fn removeBlock(index) {
    editorBlocks = editorBlocks.filter((_, i) => i !== index);
  }

  // 5. Atomic PUT Save Request to SQLx database
  async fn savePageLayout() {
    saveStatus = 'Saving...';
    try {
      const res = await fetch(`${API_BASE}/admin/pages/${selectedPage.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
        },
        body: JSON.stringify({
          title: selectedPage.title,
          slug: selectedPage.slug,
          status: selectedPage.status,
          content: editorBlocks,
          meta: selectedPage.meta || {}
        })
      });
      
      if (res.ok) {
        saveStatus = 'Success! Saved and caches invalidated.';
        setTimeout(() => saveStatus = '', 3000);
        loadPages();
      } else {
        const data = await res.json();
        saveStatus = `Error: ${data.message}`;
      }
    } catch (e) {
      saveStatus = 'Failed to save page layout.';
    }
  }
</script>

<!-- App Styling -->
<style>
  .app-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2px solid #e5e7eb;
    padding-bottom: 20px;
    margin-bottom: 30px;
  }
  .card {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    padding: 24px;
    margin-bottom: 20px;
  }
  .btn {
    background: #4f46e5;
    color: #ffffff;
    border: none;
    padding: 10px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
  }
  .btn:hover { background: #4338ca; }
  .btn-danger { background: #ef4444; }
  .btn-danger:hover { background: #dc2626; }
  .btn-secondary { background: #4b5563; }
  .btn-secondary:hover { background: #374151; }

  /* Login Form */
  .login-box {
    max-width: 400px;
    margin: 100px auto;
  }
  .form-group {
    margin-bottom: 16px;
  }
  .form-group label {
    display: block;
    font-weight: 600;
    margin-bottom: 6px;
  }
  .form-group input, .form-group select {
    width: 100%;
    padding: 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    box-sizing: border-box;
  }

  /* Editor Blocks */
  .block-container {
    border: 2px dashed #cbd5e1;
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
    background: #f8fafc;
  }
  .block-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
    border-bottom: 1px solid #e2e8f0;
    padding-bottom: 8px;
    font-weight: bold;
    color: #475569;
  }
</style>

<div class="app-container">
  {#if !token}
    <!-- Login Form View -->
    <div class="card login-box">
      <h2>ForgePress Admin</h2>
      {#if authError}
        <p style="color: #ef4444;">{authError}</p>
      {/if}
      <div class="form-group">
        <label for="username">Username</label>
        <input type="text" id="username" bind:value={username} placeholder="admin" />
      </div>
      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" bind:value={password} />
      </div>
      <button class="btn" on:click={handleLogin}>Log In</button>
    </div>
  {:else}
    <!-- Main Dashboard View -->
    <div class="header">
      <h1>ForgePress Dashboard</h1>
      <button class="btn btn-secondary" on:click={handleLogout}>Log Out</button>
    </div>

    {#if activeView === 'list'}
      <!-- Page List Sub-View -->
      <div class="card">
        <h2>Your Web Pages</h2>
        {#if pages.length === 0}
          <p>No pages registered yet.</p>
        {:else}
          <table style="width: 100%; border-collapse: collapse;">
            <thead>
              <tr style="border-bottom: 2px solid #e5e7eb; text-align: left;">
                <th style="padding: 12px;">Title</th>
                <th style="padding: 12px;">Slug</th>
                <th style="padding: 12px;">Status</th>
                <th style="padding: 12px;">Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each pages as page}
                <tr style="border-bottom: 1px solid #e5e7eb;">
                  <td style="padding: 12px; font-weight: 600;">{page.title}</td>
                  <td style="padding: 12px; color: #4b5563;">/{page.slug}</td>
                  <td style="padding: 12px;">
                    <span style="background: {page.status === 'published' ? '#d1fae5' : '#fee2e2'}; color: {page.status === 'published' ? '#065f46' : '#991b1b'}; padding: 4px 8px; border-radius: 12px; font-size: 12px; font-weight: bold;">
                      {page.status}
                    </span>
                  </td>
                  <td style="padding: 12px;">
                    <button class="btn" on:click={() => openEditor(page)}>Edit Layout</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/if}
      </div>
    {:else if activeView === 'editor'}
      <!-- Visual Page Builder Sub-View -->
      <div class="card">
        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
          <h2>Editing: {selectedPage.title}</h2>
          <div>
            <button class="btn btn-secondary" on:click={() => activeView = 'list'}>Back to List</button>
            <button class="btn" on:click={savePageLayout}>Save Page</button>
          </div>
        </div>

        {#if saveStatus}
          <p style="font-weight: bold; color: #4f46e5;">{saveStatus}</p>
        {/if}

        <div class="form-group" style="display: flex; gap: 16px;">
          <div style="flex: 1;">
            <label for="title">Title</label>
            <input type="text" id="title" bind:value={selectedPage.title} />
          </div>
          <div style="flex: 1;">
            <label for="status">Publication Status</label>
            <select id="status" bind:value={selectedPage.status}>
              <option value="draft">Draft</option>
              <option value="published">Published</option>
              <option value="scheduled">Scheduled</option>
            </select>
          </div>
        </div>

        <hr style="border: 0; border-top: 1px solid #e2e8f0; margin: 30px 0;" />

        <h3>Page Layout Blocks</h3>
        
        {#each editorBlocks as block, i}
          <div class="block-container">
            <div class="block-header">
              <span>Block #{i + 1}: {block.type}</span>
              <button class="btn btn-danger" style="padding: 4px 8px; font-size: 12px;" on:click={() => removeBlock(i)}>Delete Block</button>
            </div>

            <!-- Block Settings Customizer -->
            {#if block.type === 'hero_section'}
              <div class="form-group">
                <label for="bg-{i}">Background Color</label>
                <input type="text" id="bg-{i}" bind:value={block.settings.background} />
              </div>
              <div class="form-group">
                <label for="pad-{i}">Vertical Padding</label>
                <input type="text" id="pad-{i}" bind:value={block.settings.padding} />
              </div>
            {:else if block.type === 'rich_text'}
              <div class="form-group">
                <label for="text-{i}">Content Text</label>
                <input type="text" id="text-{i}" bind:value={block.data.text} />
              </div>
            {/if}
          </div>
        {/each}

        <!-- Add Blocks Palette -->
        <div style="margin-top: 30px; border: 2px dashed #94a3b8; border-radius: 8px; padding: 20px; text-align: center;">
          <h4 style="margin: 0 0 12px 0; color: #475569;">+ Add Layout Component</h4>
          <button class="btn" style="margin-right: 10px;" on:click={addHeroBlock}>Hero Section</button>
          <button class="btn" on:click={addRichTextBlock}>Rich Text Block</button>
        </div>
      </div>
    {/if}
  {/if}
</div>