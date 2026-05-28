<!-- /admin-dashboard/src/App.svelte -->
<script>
  import { onMount } from 'svelte';

  // API State Variables
  const API_BASE = 'http://localhost:8080/api';
  let token = localStorage.getItem('fp_token') || '';
  let pages = [];
  let selectedPage = null;
  let activeTab = 'pages'; // 'pages', 'plugins', 'settings'
  let activeView = 'list'; // 'list' or 'editor'

  // Auth Forms State
  let username = '';
  let password = '';
  let authError = '';

  // Page Editor State
  let editorBlocks = [];
  let saveStatus = '';

  // Create Page State
  let newTitle = '';
  let newSlug = '';
  let createError = '';
  let showCreateModal = false;

  // Lifecycle Initialization
  onMount(() => {
    if (token) {
      loadPages();
    }
  });

  // HIGH-PERFORMANCE SAFE FETCH WRAPPER
  // Gracefully handles non-JSON responses and outputs exact server errors to the UI
  async function safeFetch(url, options = {}) {
    try {
      const res = await fetch(url, options);
      let data = null;
      const contentType = res.headers.get("content-type");
      
      if (contentType && contentType.includes("application/json")) {
        data = await res.json();
      } else {
        const text = await res.text();
        data = { message: text || `HTTP Error ${res.status}` };
      }

      return { ok: res.ok, status: res.status, data };
    } catch (e) {
      return { ok: false, status: 0, data: { message: e.message || 'Network connection failed' } };
    }
  }

  // 1. Secure Authentication Login handler
  async function handleLogin() {
    authError = '';
    const { ok, data } = await safeFetch(`${API_BASE}/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    });

    if (ok) {
      token = data.token;
      localStorage.setItem('fp_token', token);
      loadPages();
    } else {
      authError = data.message || 'Login failed.';
    }
  }

  function handleLogout() {
    token = '';
    localStorage.removeItem('fp_token');
    pages = [];
    selectedPage = null;
    activeView = 'list';
  }

  // 2. Fetch all registered page models
  async function loadPages() {
    const { ok, status, data } = await safeFetch(`${API_BASE}/admin/pages`, {
      headers: { 'Authorization': `Bearer ${token}` }
    });

    if (ok) {
      pages = data.data || [];
    } else {
      console.error('Failed to load pages metadata:', data.message);
      if (status === 401) {
        handleLogout();
      }
    }
  }

  // 3. Create Page Handler (POST Request to Rust Backend)
  async function handleCreatePage() {
    createError = '';
    if (!newTitle || !newSlug) {
      createError = 'Please fill out all fields.';
      return;
    }
    
    // Normalize slug format to lowercase/hyphens
    const normalizedSlug = newSlug.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-_/]/g, '');

    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify({ title: newTitle, slug: normalizedSlug })
    });

    if (ok) {
      newTitle = '';
      newSlug = '';
      showCreateModal = false;
      loadPages(); // Reload page list
    } else {
      // Displays the detailed, raw database/system error directly in the Svelte UI in red!
      createError = data.message || 'Failed to create page.';
    }
  }

  // 4. Open selected page in the block editor
  async function openEditor(page) {
    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages/by-slug/${page.slug}`, {
      headers: { 'Authorization': `Bearer ${token}` }
    });

    if (ok) {
      selectedPage = data.data;
      editorBlocks = typeof selectedPage.content === 'string' 
        ? JSON.parse(selectedPage.content) 
        : selectedPage.content || [];
      activeView = 'editor';
    } else {
      console.error('Failed to fetch full page layout:', data.message);
    }
  }

  // 5. Page Builder Functions
  function addHeroBlock() {
    editorBlocks = [...editorBlocks, {
      type: "hero_section",
      settings: { background: "#4f46e5", padding: "80px" },
      blocks: [
        { type: "heading", data: { text: "Editable Hero Heading", level: 1 } }
      ]
    }];
  }

  function addRichTextBlock() {
    editorBlocks = [...editorBlocks, {
      type: "rich_text",
      settings: { padding: "20px" },
      data: { text: "This is a new, editable rich text paragraph." }
    }];
  }

  function removeBlock(index) {
    editorBlocks = editorBlocks.filter((_, i) => i !== index);
  }

  // 6. Atomic PUT Save Request to SQLx database
  async function savePageLayout() {
    saveStatus = 'Saving...';
    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages/${selectedPage.id}`, {
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
    
    if (ok) {
      saveStatus = 'Success! Saved and caches invalidated.';
      setTimeout(() => saveStatus = '', 3000);
      loadPages();
    } else {
      saveStatus = `Error: ${data.message}`;
    }
  }
</script>

<!-- App Layout Styling -->
<style>
  .app-layout {
    display: flex;
    min-height: 100vh;
  }
  .sidebar {
    width: 260px;
    background-color: #1f2937;
    color: #f3f4f6;
    padding: 20px;
    box-sizing: border-box;
  }
  .sidebar h2 {
    margin-top: 0;
    font-size: 22px;
    border-bottom: 1px solid #374151;
    padding-bottom: 15px;
    color: #6366f1;
  }
  .nav-list {
    list-style: none;
    padding: 0;
    margin: 20px 0;
  }
  .nav-item {
    padding: 12px 16px;
    border-radius: 6px;
    cursor: pointer;
    margin-bottom: 8px;
    font-weight: 600;
    transition: background 0.2s;
  }
  .nav-item:hover, .nav-item.active {
    background-color: #374151;
    color: #ffffff;
  }
  .main-content {
    flex: 1;
    background-color: #f3f4f6;
    padding: 30px;
    box-sizing: border-box;
  }
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
  }
  .btn:hover { background: #4338ca; }
  .btn-danger { background: #ef4444; }
  .btn-danger:hover { background: #dc2626; }
  .btn-secondary { background: #4b5563; }
  .btn-secondary:hover { background: #374151; }

  /* Forms & Dialogs */
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

  /* Block Editor */
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

<div class="app-layout">
  {#if !token}
    <!-- Secure Login View -->
    <div class="card login-box">
      <h2 style="color: #4f46e5; margin-top:0;">ForgePress Admin</h2>
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
      <button class="btn" style="width: 100%;" on:click={handleLogin}>Log In</button>
    </div>
  {:else}
    <!-- Main Admin Workspace Layout -->
    <div class="app-layout" style="width: 100%;">
      <!-- Left Sidebar -->
      <div class="sidebar">
        <h2>ForgePress</h2>
        <ul class="nav-list">
          <li class="nav-item {activeTab === 'pages' ? 'active' : ''}" on:click={() => { activeTab = 'pages'; activeView = 'list'; selectedPage = null; }}>Pages</li>
          <li class="nav-item {activeTab === 'plugins' ? 'active' : ''}" on:click={() => { activeTab = 'plugins'; activeView = 'list'; }}>Plugins</li>
          <li class="nav-item {activeTab === 'settings' ? 'active' : ''}" on:click={() => { activeTab = 'settings'; activeView = 'list'; }}>Settings</li>
        </ul>
        <button class="btn btn-danger" style="width: 100%; margin-top: 40px;" on:click={handleLogout}>Log Out</button>
      </div>

      <!-- Main Content Area -->
      <div class="main-content">
        {#if activeTab === 'pages'}
          <!-- PAGES MODULE -->
          {#if activeView === 'list'}
            <!-- Pages List Sub-View -->
            <div class="header">
              <h1 style="margin:0;">Web Pages</h1>
              <button class="btn" on:click={() => showCreateModal = true}>+ Create New Page</button>
            </div>

            {#if showCreateModal}
              <!-- Simple Create Page Form Card -->
              <div class="card" style="border-left: 4px solid #4f46e5;">
                <h3 style="margin-top:0;">Create New Page</h3>
                {#if createError}
                  <p style="color: #ef4444; font-weight: bold; background-color: #fee2e2; padding: 10px; border-radius: 6px;">{createError}</p>
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
          {:else if activeView === 'editor'}
            <!-- Visual Page Builder Sub-View -->
            <div class="card">
              <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
                <h2 style="margin:0;">Layout Editor: {selectedPage.title}</h2>
                <div>
                  <button class="btn btn-secondary" style="margin-right: 10px;" on:click={() => activeView = 'list'}>Back to List</button>
                  <button class="btn" on:click={savePageLayout}>Save Page</button>
                </div>
              </div>

              {#if saveStatus}
                <p style="font-weight: bold; color: #4f46e5; background: #e0e7ff; padding: 10px; border-radius: 6px;">{saveStatus}</p>
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

              <h3>Visual Layout Components</h3>
              
              {#each editorBlocks as block, i}
                <div class="block-container">
                  <div class="block-header">
                    <span>Block #{i + 1}: {block.type}</span>
                    <button class="btn btn-danger" style="padding: 4px 10px; font-size: 12px;" on:click={() => removeBlock(i)}>Delete Component</button>
                  </div>

                  <!-- Custom Block Customizer -->
                  {#if block.type === 'hero_section'}
                    <div class="form-group">
                      <label for="bg-{i}">Background Color (HEX)</label>
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
              <div style="margin-top: 30px; border: 2px dashed #94a3b8; border-radius: 8px; padding: 24px; text-align: center; background-color: #f8fafc;">
                <h4 style="margin: 0 0 16px 0; color: #475569; font-size: 16px;">+ Add Visual Layout Component</h4>
                <button class="btn" style="margin-right: 12px;" on:click={addHeroBlock}>Hero Section</button>
                <button class="btn" on:click={addRichTextBlock}>Rich Text Block</button>
              </div>
            </div>
          {/if}

        {:else if activeTab === 'plugins'}
          <!-- PLUGINS MODULE -->
          <div class="header">
            <h1 style="margin:0;">Sandboxed Plugins</h1>
          </div>
          <div class="card">
            <p style="color: #4b5563; margin-bottom: 20px;">
              Active extensions parsed dynamically from your server's <code>/content/plugins</code> directory.
            </p>

            <table style="width: 100%; border-collapse: collapse;">
              <thead>
                <tr style="border-bottom: 2px solid #e5e7eb; text-align: left;">
                  <th style="padding: 12px;">Extension Name</th>
                  <th style="padding: 12px;">Type</th>
                  <th style="padding: 12px;">Author</th>
                  <th style="padding: 12px;">Status</th>
                </tr>
              </thead>
              <tbody>
                <tr style="border-bottom: 1px solid #e5e7eb;">
                  <td style="padding: 12px; font-weight: 600;">Contact Forms Builder</td>
                  <td style="padding: 12px; color: #4b5563;">Rhai Script Hook</td>
                  <td style="padding: 12px; color: #4b5563;">System Core</td>
                  <td style="padding: 12px;">
                    <span style="background: #d1fae5; color: #065f46; padding: 4px 10px; border-radius: 12px; font-size: 12px; font-weight: bold;">
                      Active (Sandboxed)
                    </span>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

        {:else if activeTab === 'settings'}
          <!-- SETTINGS MODULE -->
          <div class="header">
            <h1 style="margin:0;">Themes & Global Settings</h1>
          </div>
          <div class="card">
            <h3 style="margin-top:0;">Active Theme</h3>
            <div style="display: flex; gap: 20px; align-items: center; background: #f8fafc; padding: 16px; border-radius: 8px; border: 1px solid #e2e8f0; margin-bottom: 30px;">
              <div style="background: #4f46e5; color: white; width: 60px; height: 60px; display: flex; align-items: center; justify-content: center; border-radius: 8px; font-weight: bold; font-size: 24px;">T</div>
              <div>
                <h4 style="margin: 0 0 4px 0;">Default Theme</h4>
                <p style="margin: 0; color: #64748b; font-size: 14px;">Version 1.0.0 | By Admin | Compatible with GrapesJS Builder</p>
              </div>
            </div>

            <hr style="border:0; border-top: 1px solid #e2e8f0; margin-bottom: 24px;" />

            <h3>System Settings</h3>
            <div class="form-group">
              <label for="site-title">Site Name</label>
              <input type="text" id="site-title" value="My ForgePress Web Engine" disabled />
            </div>
            <div class="form-group">
              <label for="site-desc">Tagline</label>
              <input type="text" id="site-desc" value="Decoupled dynamic web publishing, forged in Rust." disabled />
            </div>
            <p style="color: #64748b; font-size: 13px;">* Global options are configured directly inside your configuration environment (.env).</p>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>