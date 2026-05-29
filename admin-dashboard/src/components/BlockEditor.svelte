<!-- /admin-dashboard/src/components/BlockEditor.svelte -->
<script>
  export let selectedPage;
  export let editorBlocks = [];
  export let activeView;
  export let blockRegistry = []; 
  export let savePageLayout;
  export let saveStatus = '';

  let selectedBlockIndex = null; // Tracks which block is currently selected for styles
  let sidebarTab = 'blocks'; // ADDED: Elementor-style Sidebar Tabs ('blocks', 'navigator', 'settings', 'seo')

  // Dynamically adds any block schema configuration array element
  function addBlock(blockType) {
    const schema = blockRegistry.find(b => b.type === blockType);
    if (!schema) return;

    const settings = {};
    schema.settings.forEach(field => {
      settings[field.key] = field.default;
    });

    const data = {};
    schema.data.forEach(field => {
      data[field.key] = field.default;
    });

    let blocks = [];
    if (blockType === "hero_section") {
      blocks = [
        { type: "heading", data: { text: "Editable Hero Heading", level: 1 } }
      ];
    }

    editorBlocks = [...editorBlocks, {
      type: blockType,
      settings,
      data,
      blocks
    }];
    selectedBlockIndex = editorBlocks.length - 1;
    sidebarTab = 'navigator'; // Automatically focus navigator to customize settings
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
    sidebarTab = 'navigator'; // Focus navigator/styling on canvas click
  }

  // Translates block JSON schemas settings properties into native inline CSS!
  function getBlockStyles(block) {
    let styles = [];
    if (!block.settings) return "";
    
    if (block.settings.background) styles.push(`background-color: ${block.settings.background}`);
    if (block.settings.color) styles.push(`color: ${block.settings.color}`);
    if (block.settings.font_size) styles.push(`font-size: ${block.settings.font_size}px`);
    if (block.settings.text_align) styles.push(`text-align: ${block.settings.text_align}`);
    if (block.settings.padding_v) styles.push(`padding-top: ${block.settings.padding_v}px; padding-bottom: ${block.settings.padding_v}px`);
    if (block.settings.padding_h) styles.push(`padding-left: ${block.settings.padding_h}px; padding-right: ${block.settings.padding_h}px`);
    if (block.settings.margin_v) styles.push(`margin-top: ${block.settings.margin_v}px; margin-bottom: ${block.settings.margin_v}px`);
    if (block.settings.flex_direction) styles.push(`display: flex; flex-direction: ${block.settings.flex_direction}`);
    if (block.settings.justify_content) styles.push(`justify-content: ${block.settings.justify_content}`);
    if (block.settings.align_items) styles.push(`align-items: ${block.settings.align_items}`);
    if (block.settings.radius) styles.push(`border-radius: ${block.settings.radius}px`);
    if (block.settings.width) styles.push(`max-width: ${block.settings.width}%`);
    
    return styles.join("; ");
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
    
    <!-- ELEMENTOR STYLE SIDEBAR TABS -->
    <div class="sidebar-tabs">
      <button class={sidebarTab === 'blocks' ? 'active' : ''} on:click={() => sidebarTab = 'blocks'}>
        🧩 Widgets
      </button>
      <button class={sidebarTab === 'navigator' ? 'active' : ''} on:click={() => sidebarTab = 'navigator'}>
        🌳 Navigator
      </button>
      <button class={sidebarTab === 'settings' ? 'active' : ''} on:click={() => sidebarTab = 'settings'}>
        ⚙️ Settings
      </button>
      <button class={sidebarTab === 'seo' ? 'active' : ''} on:click={() => sidebarTab = 'seo'}>
        🔍 SEO
      </button>
    </div>

    <!-- TAB 1: ADD ELEMENTS DRAWER -->
    {#if sidebarTab === 'blocks'}
      <div class="tab-content">
        <div class="inspector-section" style="border: none;">
          <h3>Add Elements</h3>
          {#each ['Layout', 'Basic Elements', 'Media'] as category}
            <div class="drawer-category">
              <h4>{category}</h4>
              <div class="palette">
                {#each blockRegistry.filter(b => b.category === category) as blockSchema}
                  <button class="palette-btn" on:click={() => addBlock(blockSchema.type)}>
                    + {blockSchema.name}
                  </button>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </div>

    <!-- TAB 2: ACTIVE ELEMENTS HIERARCHY & COMPONENT STYLING -->
    {:else if sidebarTab === 'navigator'}
      <div class="tab-content">
        <div class="inspector-section">
          <h3>Layout Navigator</h3>
          {#if editorBlocks.length === 0}
            <p style="color: #94a3b8; font-size: 13px;">No elements in layout yet. Add some in the Widgets tab!</p>
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

        <div class="inspector-section" style="border: none;">
          <h3>Style & Properties</h3>
          {#if selectedBlockIndex === null}
            <p style="color: #94a3b8; font-size: 13px; text-align: center; padding: 20px 0;">
              Select a component on the live canvas or layout navigator to edit styles.
            </p>
          {:else}
            {@const block = editorBlocks[selectedBlockIndex]}
            {@const schema = blockRegistry.find(b => b.type === block.type)}
            
            <div class="style-group">
              <span class="active-tag">Editing: {schema ? schema.name : block.type}</span>

              {#if schema}
                <!-- Render Custom Content/Data Fields Dynamically -->
                {#if schema.data && schema.data.length > 0}
                  <h4 style="margin: 0 0 10px 0; font-size: 11px; color: #64748b; text-transform: uppercase;">Content Data</h4>
                  {#each schema.data as field}
                    <div class="form-group">
                      <label for="data-{field.key}">{field.label}</label>
                      {#if field.type === 'textarea'}
                        <textarea id="data-{field.key}" rows="4" bind:value={editorBlocks[selectedBlockIndex].data[field.key]}></textarea>
                      {:else}
                        <input type="text" id="data-{field.key}" bind:value={editorBlocks[selectedBlockIndex].data[field.key]} />
                      {/if}
                    </div>
                  {/each}
                {/if}

                <!-- Render Style Settings Fields Dynamically -->
                {#if schema.settings && schema.settings.length > 0}
                  <h4 style="margin: 15px 0 10px 0; font-size: 11px; color: #64748b; text-transform: uppercase;">Style Settings</h4>
                  {#each schema.settings as field}
                    <div class="form-group">
                      <label for="set-{field.key}">
                        {field.label}
                        {#if field.type === 'range'}
                          : {editorBlocks[selectedBlockIndex].settings[field.key]}
                        {/if}
                      </label>

                      {#if field.type === 'color'}
                        <div style="display: flex; gap: 10px; align-items: center;">
                          <input type="color" id="set-{field.key}" bind:value={editorBlocks[selectedBlockIndex].settings[field.key]} style="width: 50px; height: 35px; border: none; cursor: pointer; background: none;" />
                          <input type="text" bind:value={editorBlocks[selectedBlockIndex].settings[field.key]} style="flex: 1;" />
                        </div>
                      {:else if field.type === 'range'}
                        <input type="range" id="set-{field.key}" min={field.min} max={field.max} step={field.step} bind:value={editorBlocks[selectedBlockIndex].settings[field.key]} />
                      {:else if field.type === 'select'}
                        <select id="set-{field.key}" bind:value={editorBlocks[selectedBlockIndex].settings[field.key]}>
                          {#each field.options as option}
                            <option value={option.value}>{option.label}</option>
                          {/each}
                        </select>
                      {/if}
                    </div>
                  {/each}
                {/if}
              {:else}
                <p style="color: #ef4444; font-size: 12px;">Unknown block type schema configuration.</p>
              {/if}
            </div>
          {/if}
        </div>
      </div>

    <!-- TAB 3: GENERAL PAGE ATTRIBUTES -->
    {:else if sidebarTab === 'settings'}
      <div class="tab-content">
        <div class="inspector-section" style="border: none;">
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
      </div>

    <!-- TAB 4: SEO METADATA BUILDER -->
    {:else if sidebarTab === 'seo'}
      <div class="tab-content">
        <div class="inspector-section" style="border: none;">
          <h3>SEO & Metadata</h3>
          <div class="form-group">
            <label for="seo_title">SEO Browser Title</label>
            <input type="text" id="seo_title" bind:value={selectedPage.meta.seo_title} placeholder="e.g. My Page Title | ForgePress" />
          </div>
          <div class="form-group">
            <label for="seo_desc">Meta Description</label>
            <textarea id="seo_desc" rows="5" bind:value={selectedPage.meta.seo_description} placeholder="Enter page search result snippet summary..."></textarea>
          </div>
          <div class="form-group" style="margin-bottom: 20px;">
            <label for="seo_img">Social Share Preview Image URL</label>
            <input type="text" id="seo_img" bind:value={selectedPage.meta.social_image} placeholder="https://..." />
          </div>
        </div>
      </div>
    {/if}
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
              
              {#if block.type === 'spacer'}
                <!-- Transparent helper spacer rendering preview inside visual canvas -->
                <div style="height: {block.settings.height || '40'}px; width: 100%; background: repeating-linear-gradient(45deg, #f1f5f9, #f1f5f9 10px, #fafafa 10px, #fafafa 20px); border: 1px dashed #cbd5e1; box-sizing: border-box;">
                  <span style="font-size: 10px; color: #94a3b8; padding: 4px; display: block; text-align: center; font-family: monospace;">Spacer ({block.settings.height || '40'}px)</span>
                </div>

              {:else if block.type === 'divider'}
                <div style="margin: {block.settings.margin_v || '20'}px auto; width: {block.settings.width || '60'}%; text-align: center;">
                  <hr style="
                    border: none;
                    border-top: {block.settings.thickness || '2'}px solid {block.settings.color || '#cbd5e1'};
                    margin: 0 auto;
                  " />
                </div>

              {:else if block.type === 'image'}
                <div style="text-align: {block.settings.align || 'center'}; margin: {block.settings.margin_v || '15'}px 0; width: 100%; box-sizing: border-box; padding: 0 20px;">
                  <img src={block.data.url || 'https://picsum.photos/400/200'} alt={block.data.alt || 'Visual image'} style="
                    max-width: {block.settings.width || '100'}%;
                    height: auto;
                    border-radius: {block.settings.radius || '8'}px;
                    display: inline-block;
                  " />
                </div>

              {:else if block.type === 'video'}
                <div style="margin: 20px auto; width: {block.settings.width || '100'}%; max-width: 600px; padding: 0 20px; box-sizing: border-box;">
                  <div style="position: relative; padding-bottom: 56.25%; height: 0; background: #334155; border-radius: 8px; overflow: hidden; display: flex; align-items: center; justify-content: center; border: 1px dashed #cbd5e1;">
                    <span style="position: absolute; color: #cbd5e1; font-size: 12px; font-weight: bold; font-family: monospace;">Embedded Video Viewport</span>
                  </div>
                </div>

              {:else if block.type === 'button'}
                <div style="text-align: {block.settings.align || 'center'}; margin: {block.settings.margin_v || '15'}px 0; width: 100%; box-sizing: border-box; padding: 0 20px;">
                  <span style="
                    background-color: {block.settings.bg || '#4f46e5'};
                    color: {block.settings.color || '#ffffff'};
                    border-radius: {block.settings.radius || '6'}px;
                    padding: {block.settings.padding_v || '10'}px {block.settings.padding_h || '20'}px;
                    display: inline-block;
                    font-weight: bold;
                    text-decoration: none;
                  ">
                    {block.data.text || 'Click Here'}
                  </span>
                </div>

              {:else if block.type === 'hero_section'}
                <!-- Dynamic block styling renders the layout automatically based on settings -->
                <div style="{getBlockStyles(block)}; width: 100%; box-sizing: border-box; transition: all 0.2s ease-out; gap: 15px;">
                  <h1 style="margin: 0; font-size: 32px; font-weight: 800; text-align: center;">
                    {block.blocks?.[0]?.data?.text || 'Editable Hero Heading'}
                  </h1>
                </div>

              {:else}
                <!-- UNIVERSAL STYLED BLOCK FALLBACK -->
                <div style="{getBlockStyles(block)}; width: 100%; box-sizing: border-box; transition: all 0.2s ease-out; padding: 10px 40px;">
                  {#if block.data && block.data.text}
                    {block.data.text}
                  {:else}
                    <div style="border: 1px dashed #cbd5e1; padding: 20px; text-align: center; border-radius: 6px; background: #f8fafc; color: #64748b;">
                      <strong>{block.type}</strong> - Click to customize properties in sidebar
                    </div>
                  {/if}
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
    height: 61px;
    box-sizing: border-box;
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

  /* Workspace Splitscreen layout (100% height fill) */
  .workspace {
    display: flex;
    height: calc(100vh - 61px); /* Occupy 100% remaining vertical screen space */
    background: #0f172a;
    border-radius: 0;
    overflow: hidden;
  }

  /* Left Inspector Panel (Tabbed layout) */
  .sidebar-inspector {
    width: 340px;
    background: #1e293b;
    border-right: 1px solid #334155;
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  
  /* ELEMENTOR TAB STYLING */
  .sidebar-tabs {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    background: #0f172a;
    border-bottom: 1px solid #334155;
    position: sticky;
    top: 0;
    z-index: 20;
  }
  .sidebar-tabs button {
    background: transparent;
    color: #94a3b8;
    border: none;
    padding: 12px 2px;
    font-size: 10px;
    font-weight: 700;
    cursor: pointer;
    text-align: center;
    transition: all 0.2s;
    border-bottom: 2px solid transparent;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }
  .sidebar-tabs button:hover {
    color: #f1f5f9;
    background: #1e293b;
  }
  .sidebar-tabs button.active {
    color: #38bdf8;
    border-bottom-color: #38bdf8;
    background: #1e293b;
  }

  .tab-content {
    flex: 1;
    overflow-y: auto;
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

  /* Category Dividers */
  .drawer-category {
    margin-bottom: 16px;
  }
  .drawer-category h4 {
    margin: 0 0 10px 0;
    font-size: 11px;
    color: #64748b;
    text-transform: uppercase;
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
    font-size: 11px;
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
    font-size: 10px;
    font-weight: bold;
    transition: all 0.2s;
  }
  .palette-btn:hover {
    border-color: #4f46e5;
    color: #818cf8;
    background: #1e1b4b;
  }

  /* Right Panel Viewport simulated screen styles (STRETCHED TO REMOVE BOTTOM CUTOFF) */
  .editor-canvas-container {
    flex: 1;
    padding: 30px;
    background: #0f172a;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    overflow-y: auto;
    height: 100%;
    box-sizing: border-box;
  }
  .browser-frame {
    width: 100%;
    max-width: 1024px;
    min-height: calc(100% - 10px); /* Stretch dynamically to fill the container height */
    background: white;
    border-radius: 8px;
    box-shadow: 0 20px 25px -5px rgba(0,0,0,0.5);
    overflow: hidden;
    border: 1px solid #334155;
    display: flex;
    flex-direction: column;
  }
  .browser-header {
    background: #e2e8f0;
    padding: 10px 20px;
    display: flex;
    align-items: center;
    border-bottom: 1px solid #cbd5e1;
    height: 41px;
    box-sizing: border-box;
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
    flex-grow: 1; /* Expand viewport to absorb any remaining browser frame height */
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