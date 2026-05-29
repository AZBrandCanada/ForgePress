<!-- /admin-dashboard/src/App.svelte -->
<script>
  import { onMount } from 'svelte';
  import Login from './components/Login.svelte';
  import Sidebar from './components/Sidebar.svelte';
  import PageList from './components/PageList.svelte';
  import BlockEditor from './components/BlockEditor.svelte';

  // API State Variables
  const API_BASE = 'http://localhost:8080/api';
  let token = localStorage.getItem('fp_token') || '';
  let pages = [];
  let blockRegistry = []; // Dynamically populated from backend scan
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
  let editorError = ''; // Specific validation for editor loading

  // Create Page State
  let newTitle = '';
  let newSlug = '';
  let createError = '';
  let showCreateModal = false;

  onMount(() => {
    if (token) {
      loadPages();
      loadBlockRegistry(); // Load blocks dynamically
    }
  });

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

  // Dynamically fetch blocks from Rust server theme scan
  async function loadBlockRegistry() {
    const { ok, data } = await safeFetch(`${API_BASE}/admin/themes/blocks`, {
      headers: { 'Authorization': `Bearer ${token}` }
    });
    if (ok) {
      blockRegistry = data.data || [];
    } else {
      console.error('Failed to load dynamic block schemas:', data.message);
    }
  }

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
      loadBlockRegistry(); // Load blocks on login
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

  async function handleCreatePage() {
    createError = '';
    
    // We only require a Title. The Slug can now be empty for the homepage!
    if (!newTitle) {
      createError = 'Please fill out a page title.';
      return;
    }
    
    // Normalize slug: if they type "/" or leave it blank, treat as root homepage ("")
    let normalizedSlug = newSlug.trim().toLowerCase();
    if (normalizedSlug === '/' || normalizedSlug === '') {
      normalizedSlug = '';
    } else {
      normalizedSlug = normalizedSlug
        .replace(/\s+/g, '-')
        .replace(/[^a-z0-9-_/]/g, '');
    }

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
      loadPages();
    } else {
      createError = data.message || 'Failed to create page.';
    }
  }

  async function handleDeletePage(id) {
    if (!confirm('Are you sure you want to permanently delete this page?')) {
      return;
    }

    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages/${id}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });

    if (ok) {
      loadPages(); // Refresh the list
    } else {
      editorError = data.message || 'Failed to delete page.';
    }
  }

  async function handleSetHomepage(id) {
    if (!confirm('Are you sure you want to set this page as the active homepage? The existing homepage will be reverted back to a draft.')) {
      return;
    }

    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages/${id}/set-homepage`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`
      }
    });

    if (ok) {
      loadPages(); // Refresh list structure
    } else {
      editorError = data.message || 'Failed to reconfigure homepage options.';
    }
  }

  async function openEditor(page) {
    editorError = '';
    
    // Helper function to safely parse dynamic JSONB payloads and ensure SEO fields exist
    const parsePagePayload = (p) => {
      p.content = typeof p.content === 'string' ? JSON.parse(p.content) : p.content || [];
      if (!p.meta) {
        p.meta = {};
      } else if (typeof p.meta === 'string') {
        try {
          p.meta = JSON.parse(p.meta);
        } catch (e) {
          p.meta = {};
        }
      }
      p.meta.seo_title = p.meta.seo_title || '';
      p.meta.seo_description = p.meta.seo_description || '';
      p.meta.social_image = p.meta.social_image || '';
      return p;
    };

    if (page.slug === '') {
      selectedPage = parsePagePayload({ ...page });
      editorBlocks = selectedPage.content;
      activeView = 'editor';
      return;
    }
    
    // Attempt secondary slug fetch for normal pages
    const { ok, data } = await safeFetch(`${API_BASE}/admin/pages/by-slug/${page.slug}`, {
      headers: { 'Authorization': `Bearer ${token}` }
    });

    if (ok) {
      selectedPage = parsePagePayload(data.data);
      editorBlocks = selectedPage.content;
      activeView = 'editor';
    } else {
      console.warn('Failed API fetch for slug, falling back to local page cache...', data.message);
      selectedPage = parsePagePayload({ ...page });
      editorBlocks = selectedPage.content;
      activeView = 'editor';
    }
  }

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

<style>
  .app-layout {
    display: flex;
    min-height: 100vh;
  }
  .main-content {
    flex: 1;
    background-color: #f3f4f6;
    padding: 30px;
    box-sizing: border-box;
    transition: all 0.15s ease-out;
  }
</style>

<div class="app-layout">
  {#if !token}
    <Login {handleLogin} bind:username bind:password {authError} />
  {:else}
    <div class="app-layout" style="width: 100%;">
      <!-- CONDITIONALLY HIDES SIDEBAR: District-free layout editor screen -->
      {#if activeView !== 'editor'}
        <Sidebar bind:activeTab bind:activeView bind:selectedPage {handleLogout} />
      {/if}

      <!-- DYNAMIC WRAPPER STYLES: Removes padding and matches workspace dark mode if inside editor -->
      <div class="main-content" style={activeView === 'editor' ? 'padding: 0; background: #0f172a;' : ''}>
        {#if activeTab === 'pages'}
          {#if activeView === 'list'}
            <PageList 
              {pages} 
              {openEditor} 
              {handleCreatePage} 
              {handleDeletePage} 
              {handleSetHomepage}
              bind:showCreateModal 
              bind:newTitle 
              bind:newSlug 
              {createError} 
              {editorError} 
            />
          {:else if activeView === 'editor'}
            <BlockEditor 
              bind:selectedPage 
              bind:editorBlocks 
              bind:activeView 
              {blockRegistry} 
              {savePageLayout} 
              {saveStatus} 
            />
          {/if}

        {:else if activeTab === 'plugins'}
          <div class="card">
            <h1>Sandboxed Plugins</h1>
            <p>Active extensions parsed dynamically from your server's <code>/content/plugins</code> directory.</p>
          </div>

        {:else if activeTab === 'settings'}
          <div class="card">
            <h1>Themes & Global Settings</h1>
            <p>Active Theme: Default Theme | Version 1.0.0</p>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>