<!-- /admin-dashboard/src/components/Login.svelte -->
<script>
  export let handleLogin;
  export let username = '';
  export let password = '';
  export let authError = '';
</script>

<div class="card login-box">
  <h2 style="color: #4f46e5; margin-top:0;">ForgePress Admin</h2>
  {#if authError}
    <p style="color: #ef4444; font-weight: bold; background-color: #fee2e2; padding: 10px; border-radius: 6px;">{authError}</p>
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

<style>
  .card {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    padding: 24px;
    margin-bottom: 20px;
  }
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
  .form-group input {
    width: 100%;
    padding: 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    box-sizing: border-box;
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
</style>