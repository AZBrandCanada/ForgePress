<!-- /admin-dashboard/src/components/BlockEditor.svelte -->
<script>
  export let selectedPage;
  export let editorBlocks = [];
  export let activeView;
  export let savePageLayout;
  export let saveStatus = '';

  let selectedBlockIndex = null; // Tracks which block is currently selected for styles

  function addHeroBlock() {
    editorBlocks = [...editorBlocks, {
      type: "hero_section",
      settings: { 
        background: "#4f46e5", 
        padding_v: "80", 
        padding_h: "20",
        flex_direction: "column",
        justify_content: "center",
        align_items: "center"
      },
      blocks: [
        { type: "heading", data: { text: "Editable Hero Heading", level: 1 } }
      ]
    }];
    selectedBlockIndex = editorBlocks.length - 1; // Auto-select new block
  }

  function addRichTextBlock() {
    editorBlocks = [...editorBlocks, {
      type: "rich_text",
      settings: { 
        font_size: "16", 
        text_align: "left", 
        color: "#333333",
        margin_v: "15"
      },
      data: { text: "This is a new, editable rich text paragraph." }
    }];
    selectedBlockIndex = editorBlocks.length - 1; // Auto-select new block
  }

  function removeBlock(index) {
    editorBlocks = editorBlocks.filter((_, i) => i !== index);
    if (selectedBlockIndex === index) {
      selectedBlockIndex = null;
    } else if (selectedBlockIndex > index) {
      selectedBlockIndex--;
    }
  }

  function moveBlockUp(index) {
    if (index === 0) return;
    const temp = editorBlocks[index];
    editorBlocks[index] = editorBlocks[index - 1];
    editorBlocks[index - 1] = temp;
    editorBlocks = [...editorBlocks];
    selectedBlockIndex = index - 1;
  }

  // Uses temporary local swap to safely switch active array locations
  function moveBlockDown(index) {
    if (index === editorBlocks.length - 1) return;
    const temp = editorBlocks[index];
    editorBlocks[index] = editorBlocks[index + 1];
    editorBlocks[index + 1] = temp;
    editorBlocks = [...editorBlocks];
    selectedBlockIndex = index + 1;
  }

  function selectBlock(index) {
    selectedBlockIndex = index;
  }
</script>

<div class="editor-header">
  <div class="header-left">
    <h2 style="margin:0; font-size: 18px;">Page: {selectedPage.title}</h2>
    {#if saveStatus}
      <span class="save-status">{saveStatus}</span>
    {/if}
  </div>
  <div class="header-right">
    <button class="btn btn-secondary" on:click={() => activeView = 'list'}>Back to List</button>
    <button class="btn" on:click={savePageLayout} style="background: #4f46e5; color: white;">Save Page</button>
  </div>
</div>

<div class="workspace">
  <!-- LEFT SIDEBAR: STYLE & CONTENT INSPECTOR -->
  <div class="sidebar-inspector">
    <div class="inspector-section">
      <h3>Page Settings</h3>
      <div class="form-group">
        <label for="title">Title</label>
        <input type="text" id="title" bind:value={selectedPage.title} />
      </div>
      <div class="form-group">
        <label for="slug">URL Slug</label>
        <input type="text" id="slug" bind:value={selectedPage.slug} />
      </div>
      <div class="form-group">
        <label for="status">Publication Status</label>
        <select id="status" bind:value={selectedPage.status}>
          <option value="draft">Draft</option>
          <option value="published">Published</option>
          <option value="scheduled">Scheduled</option>
        </select>
      </div>
    </div>

    <!-- Active Elements Hierarchy -->
    <div class="inspector-section">
      <h3>Layout Navigator</h3>
      {#if editorBlocks.length === 0}
        <p style="color: #94a3b8; font-size: 13px;">No elements in layout yet. Add one below!</p>
      {:else}
        <ul class="block-list">
          {#each editorBlocks as block, i}
            <li class="block-item {selectedBlockIndex === i ? 'active' : ''}" on:click={() => selectBlock(i)}>
              <span class="item-name">{block.type}</span>
              <div class="item-actions">
                <button on:click|stopPropagation={() => moveBlockUp(i)} disabled={i === 0}>▲</button>
                <button on:click|stopPropagation={() => moveBlockDown(i)} disabled={i === editorBlocks.length - 1}>▼</button>
                <button class="delete-btn" on:click|stopPropagation={() => removeBlock(i)}>✕</button>
              </div>
            </li>
          {/each}
        </ul>
      {/if}
    </div>

    <!-- Styling Inspector (Binds directly to Array Element for reactive mutations) -->
    <div class="inspector-section">
      <h3>Style & Properties</h3>
      {#if selectedBlockIndex === null}
        <p style="color: #94a3b8; font-size: 13px; text-align: center; padding: 20px 0;">
          Select a component on the live canvas or layout navigator to edit styles.
        </p>
      {:else}
        <div class="style-group">
          <span class="active-tag">Editing: {editorBlocks[selectedBlockIndex].type}</span>

          <!-- HERO SECTION FLEXBOX PROPERTIES -->
          {#if editorBlocks[selectedBlockIndex].type === 'hero_section'}
            <div class="form-group">
              <label for="bg">Background Color</label>
              <div style="display: flex; gap: 10px; align-items: center;">
                <input type="color" id="bg" bind:value={editorBlocks[selectedBlockIndex].settings.background} style="width: 50px; height: 35px; border: none; cursor: pointer;" />
                <input type="text" bind:value={editorBlocks[selectedBlockIndex].settings.background} style="flex: 1;" />
              </div>
            </div>

            <div class="form-group">
              <label for="pad_v">Vertical Padding: {editorBlocks[selectedBlockIndex].settings.padding_v}px</label>
              <input type="range" id="pad_v" min="10" max="200" step="5" bind:value={editorBlocks[selectedBlockIndex].settings.padding_v} />
            </div>

            <div class="form-group">
              <label for="pad_h">Horizontal Padding: {editorBlocks[selectedBlockIndex].settings.padding_h}px</label>
              <input type="range" id="pad_h" min="0" max="100" step="5" bind:value={editorBlocks[selectedBlockIndex].settings.padding_h} />
            </div>

            <div class="form-group">
              <label for="flex_dir">Flex Direction</label>
              <select id="flex_dir" bind:value={editorBlocks[selectedBlockIndex].settings.flex_direction}>
                <option value="column">Column (Stacked)</option>
                <option value="row">Row (Side-by-Side)</option>
              </select>
            </div>

            <div class="form-group">
              <label for="justify">Justify Content (Distribution)</label>
              <select id="justify" bind:value={editorBlocks[selectedBlockIndex].settings.justify_content}>
                <option value="center">Center</option>
                <option value="flex-start">Flex Start (Top/Left)</option>
                <option value="flex-end">Flex End (Bottom/Right)</option>
                <option value="space-between">Space Between</option>
                <option value="space-around">Space Around</option>
              </select>
            </div>

            <div class="form-group">
              <label for="align">Align Items (Alignment)</label>
              <select id="align" bind:value={editorBlocks[selectedBlockIndex].settings.align_items}>
                <option value="center">Center</option>
                <option value="flex-start">Flex Start</option>
                <option value="flex-end">Flex End</option>
                <option value="stretch">Stretch</option>
              </select>
            </div>

          <!-- RICH TEXT PARAGRAPH PROPERTIES -->
          {:else if editorBlocks[selectedBlockIndex].type === 'rich_text'}
            <div class="form-group">
              <label for="content_text">Text Content</label>
              <textarea id="content_text" rows="4" bind:value={editorBlocks[selectedBlockIndex].data.text}></textarea>
            </div>

            <div class="form-group">
              <label for="color">Text Color</label>
              <div style="display: flex; gap: 10px; align-items: center;">
                <input type="color" id="color" bind:value={editorBlocks[selectedBlockIndex].settings.color} style="width: 50px; height: 35px; border: none; cursor: pointer;" />
                <input type="text" bind:value={editorBlocks[selectedBlockIndex].settings.color} style="flex: 1;" />
              </div>
            </div>

            <div class="form-group">
              <label for="f_size">Font Size: {editorBlocks[selectedBlockIndex].settings.font_size}px</label>
              <input type="range" id="f_size" min="10" max="72" step="1" bind:value={editorBlocks[selectedBlockIndex].settings.font_size} />
            </div>

            <div class="form-group">
              <label for="text_align">Text Align</label>
              <select id="text_align" bind:value={editorBlocks[selectedBlockIndex].settings.text_align}>
                <option value="left">Left</option>
                <option value="center">Center</option>
                <option value="right">Right</option>
                <option value="justify">Justified</option>
              </select>
            </div>

            <div class="form-group">
              <label for="margin_v">Vertical Margin: {editorBlocks[selectedBlockIndex].settings.margin_v}px</label>
              <input type="range" id="margin_v" min="0" max="100" step="1" bind:value={editorBlocks[selectedBlockIndex].settings.margin_v} />
            </div>
          {/if}
        </div>
      {/if}
    </div>

    <!-- Drag & Add Elements Drawer -->
    <div class="inspector-section" style="border: none;">
      <h3>Add Elements</h3>
      <div class="palette">
        <button class="palette-btn" on:click={addHeroBlock}>+ Hero Section</button>
        <button class="palette-btn" on:click={addRichTextBlock}>+ Rich Text Block</button>
      </div>
    </div>
  </div>

  <!-- RIGHT PANEL: INTERACTIVE LIVE CANVAS VIEWPORT -->
  <div class="editor-canvas-container">
    <div class="browser-frame">
      <div class="browser-header">
        <div class="browser-buttons">
          <span></span><span></span><span></span>
        </div>
        <div class="browser-address">http://127.0.0.1:8080/{selectedPage.slug === 'index' ? '' : selectedPage.slug}</div>
      </div>

      <div class="browser-viewport">
        {#if editorBlocks.length === 0}
          <div class="empty-canvas">
            <p style="font-size: 20px; font-weight: 600; color: #64748b; margin-bottom: 8px;">Your Canvas is Empty</p>
            <p style="font-size: 14px; color: #94a3b8; max-width: 320px;">Use the Left Sidebar Elements Panel to add visual layout sections to your webpage.</p>
          </div>
        {:else}
          {#each editorBlocks as block, i}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div class="canvas-block {selectedBlockIndex === i ? 'active' : ''}" on:click|stopPropagation={() => selectBlock(i)}>
              <span class="block-tag">{block.type}</span>
              
              {#if block.type === 'hero_section'}
                <div style="
                  display: flex;
                  flex-direction: {block.settings.flex_direction || 'column'};
                  justify-content: {block.settings.justify_content || 'center'};
                  align-items: {block.settings.align_items || 'center'};
                  background-color: {block.settings.background || '#4f46e5'};
                  padding: {block.settings.padding_v || '80'}px {block.settings.padding_h || '20'}px;
                  color: white;
                  width: 100%;
                  box-sizing: border-box;
                  transition: all 0.2s ease-out;
                  gap: 15px;
                ">
                  <!-- Mock Heading inside Hero layout -->
                  <h1 style="margin: 0; font-size: 32px; font-weight: 800; text-align: center;">
                    {block.blocks?.[0]?.data?.text || 'Editable Hero Heading'}
                  </h1>
                </div>

              {:else if block.type === 'rich_text'}
                <div style="
                  font-size: {block.settings.font_size || '16'}px;
                  text-align: {block.settings.text_align || 'left'};
                  color: {block.settings.color || '#333333'};
                  margin: {block.settings.margin_v || '15'}px 0;
                  padding: 10px 40px;
                  width: 100%;
                  box-sizing: border-box;
                  transition: all 0.2s ease-out;
                  line-height: 1.6;
                ">
                  {block.data.text || 'Paragraph text content details...'}
                </div>
              {/if}
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>

<style>
  /* Header Area styles */
  .editor-header {
    background: #1e293b;
    border-bottom: 1px solid #334155;
    padding: 16px 24px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    color: #f1f5f9;
    border-radius: 8px 8px 0 0;
  }
  .header-left {
    display: flex;
    align-items: center;
    gap: 15px;
  }
  .save-status {
    background: #38bdf8;
    color: #0f172a;
    font-size: 11px;
    font-weight: bold;
    padding: 4px 8px;
    border-radius: 4px;
  }
  .btn {
    border: none;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
  }
  .btn-secondary { background: #475569; color: #f1f5f9; }
  .btn-secondary:hover { background: #334155; }

  /* Workspace Splitscreen layout */
  .workspace {
    display: flex;
    height: calc(100vh - 160px);
    background: #0f172a;
    border-radius: 0 0 8px 8px;
    overflow: hidden;
  }

  /* Left Inspector Panel */
  .sidebar-inspector {
    width: 340px;
    background: #1e293b;
    border-right: 1px solid #334155;
    overflow-y: auto;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
  }
  .inspector-section {
    border-bottom: 1px solid #334155;
    padding: 20px;
  }
  .inspector-section h3 {
    margin: 0 0 16px 0;
    color: #94a3b8;
    font-size: 12px;
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  .form-group {
    margin-bottom: 14px;
  }
  .form-group label {
    display: block;
    font-size: 11px;
    font-weight: bold;
    color: #cbd5e1;
    margin-bottom: 6px;
  }
  .form-group input[type="text"], .form-group textarea, .form-group select {
    width: 100%;
    padding: 8px 12px;
    background: #0f172a;
    border: 1px solid #475569;
    border-radius: 6px;
    color: #f1f5f9;
    box-sizing: border-box;
    font-size: 13px;
  }
  .form-group input[type="range"] {
    width: 100%;
    accent-color: #4f46e5;
  }

  /* Layout Navigator List */
  .block-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .block-item {
    background: #0f172a;
    border: 1px solid #334155;
    padding: 8px 12px;
    border-radius: 6px;
    margin-bottom: 8px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: pointer;
    transition: all 0.2s;
  }
  .block-item:hover, .block-item.active {
    border-color: #4f46e5;
    background: #1e1b4b;
  }
  .item-name {
    color: #cbd5e1;
    font-size: 12px;
    font-weight: bold;
  }
  .item-actions {
    display: flex;
    gap: 4px;
  }
  .item-actions button {
    background: #334155;
    color: #cbd5e1;
    border: none;
    font-size: 8px;
    width: 18px;
    height: 18px;
    border-radius: 4px;
    cursor: pointer;
  }
  .item-actions button:hover:not(:disabled) { background: #475569; }
  .item-actions button.delete-btn { background: #991b1b; color: white; }
  .item-actions button.delete-btn:hover { background: #b91c1c; }

  /* Style Selector active block banner */
  .style-group {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .active-tag {
    background: #1e1b4b;
    color: #818cf8;
    font-size: 11px;
    font-weight: bold;
    padding: 6px 10px;
    border-radius: 4px;
    text-align: center;
    border: 1px solid #312e81;
    margin-bottom: 8px;
  }

  /* Add Elements buttons drawer */
  .palette {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }
  .palette-btn {
    background: #0f172a;
    color: #cbd5e1;
    border: 1px dashed #475569;
    padding: 12px 6px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 11px;
    font-weight: bold;
    transition: all 0.2s;
  }
  .palette-btn:hover {
    border-color: #4f46e5;
    color: #818cf8;
    background: #1e1b4b;
  }

  /* Right Panel Viewport simulated screen styles */
  .editor-canvas-container {
    flex: 1;
    padding: 30px;
    background: #0f172a;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    overflow-y: auto;
  }
  .browser-frame {
    width: 100%;
    max-width: 1024px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,0.5);
    overflow: hidden;
    border: 1px solid #334155;
  }
  .browser-header {
    background: #e2e8f0;
    padding: 10px 20px;
    display: flex;
    align-items: center;
    border-bottom: 1px solid #cbd5e1;
  }
  .browser-buttons {
    display: flex;
    gap: 6px;
    margin-right: 20px;
  }
  .browser-buttons span {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    display: inline-block;
  }
  .browser-buttons span:nth-child(1) { background: #ef4444; }
  .browser-buttons span:nth-child(2) { background: #eab308; }
  .browser-buttons span:nth-child(3) { background: #22c55e; }
  .browser-address {
    flex: 1;
    background: white;
    border-radius: 4px;
    padding: 4px 12px;
    font-size: 11px;
    color: #64748b;
    border: 1px solid #cbd5e1;
    font-family: monospace;
  }

  .browser-viewport {
    min-height: 500px;
    background: #fafafa;
    position: relative;
    display: flex;
    flex-direction: column;
  }
  .empty-canvas {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
  }

  /* Interactive canvas blocks styles */
  .canvas-block {
    position: relative;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.2s;
  }
  .canvas-block:hover {
    border: 1px dashed #38bdf8;
  }
  .canvas-block.active {
    border: 2px solid #0284c7;
  }
  .block-tag {
    position: absolute;
    top: 0;
    left: 20px;
    background: #0284c7;
    color: white;
    font-size: 9px;
    font-weight: bold;
    padding: 2px 6px;
    border-radius: 0 0 4px 4px;
    opacity: 0;
    transition: opacity 0.2s;
    z-index: 10;
  }
  .canvas-block:hover .block-tag, .canvas-block.active .block-tag {
    opacity: 1;
  }
</style>