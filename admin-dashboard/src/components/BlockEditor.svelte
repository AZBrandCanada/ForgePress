<!-- /admin-dashboard/src/components/BlockEditor.svelte -->
<script>
  export let selectedPage;
  export let editorBlocks = [];
  export let activeView;
  export let savePageLayout;
  export let saveStatus = '';

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
</script>

<div class="card">
  <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;">
    <h2 style="margin:0;">Layout Editor: {selectedPage.title}</h2>
    <div>
      <button class="btn btn-secondary" style="margin-right: 10px;" on:click={() => activeView = 'list'}>Back to List</button>
      <button class="btn" on:click={savePageLayout}>Save Page</button>
    </div>
  </div>

  {#if saveStatus}
    <p class="save-status">{saveStatus}</p>
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
  <div class="palette-container">
    <h4 style="margin: 0 0 16px 0; color: #475569; font-size: 16px;">+ Add Visual Layout Component</h4>
    <button class="btn" style="margin-right: 12px;" on:click={addHeroBlock}>Hero Section</button>
    <button class="btn" on:click={addRichTextBlock}>Rich Text Block</button>
  </div>
</div>

<style>
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
    padding: 10px 18px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
    transition: background 0.2s;
  }
  .btn:hover { background: #4338ca; }
  .btn-danger { background: #ef4444; }
  .btn-danger:hover { background: #dc2626; }
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
  .form-group input, .form-group select {
    width: 100%;
    padding: 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    box-sizing: border-box;
  }
  .save-status {
    font-weight: bold; 
    color: #4f46e5; 
    background: #e0e7ff; 
    padding: 10px; 
    border-radius: 6px;
  }
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
  .palette-container {
    margin-top: 30px; 
    border: 2px dashed #94a3b8; 
    border-radius: 8px; 
    padding: 24px; 
    text-align: center; 
    background-color: #f8fafc;
  }
</style>