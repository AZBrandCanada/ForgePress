<!-- /admin-dashboard/src/components/Sidebar.svelte -->
<script>
  export let activeTab;
  export let activeView;
  export let selectedPage;
  export let handleLogout;

  function setTab(tab) {
    activeTab = tab;
    activeView = 'list';
    selectedPage = null;
  }
</script>

<div class="sidebar">
  <h2>ForgePress</h2>
  <ul class="nav-list">
    <li class="nav-item {activeTab === 'pages' ? 'active' : ''}" on:click={() => setTab('pages')}>Pages</li>
    <li class="nav-item {activeTab === 'plugins' ? 'active' : ''}" on:click={() => setTab('plugins')}>Plugins</li>
    <li class="nav-item {activeTab === 'settings' ? 'active' : ''}" on:click={() => setTab('settings')}>Settings</li>
  </ul>
  <button class="btn btn-danger" on:click={handleLogout}>Log Out</button>
</div>

<style>
  .sidebar {
    width: 260px;
    background-color: #1f2937;
    color: #f3f4f6;
    padding: 20px;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
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
    flex-grow: 1;
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
  .btn-danger {
    background: #ef4444;
    color: #ffffff;
    border: none;
    padding: 10px 18px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
    width: 100%;
    margin-top: auto;
  }
  .btn-danger:hover { background: #dc2626; }
</style>