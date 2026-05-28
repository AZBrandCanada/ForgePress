Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "            Starting ForgePress Dev Environment           " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# 1. Ensure the .env file exists
if (-not (Test-Path ".env")) {
    Write-Host "No .env file detected. Copying .env.example to .env..." -ForegroundColor Yellow
    Copy-Item ".env.example" ".env"
}

# 2. Start the Vite Frontend in a background job
Write-Host "Starting Svelte Admin Dashboard (Vite Dev Server)..." -ForegroundColor Yellow
$ViteJob = Start-Job -ScriptBlock {
    Set-Location admin-dashboard
    npm run dev
}

# Give Vite 2 seconds to boot
Start-Sleep -Seconds 2

# 3. Run the Rust Backend in the active window
Write-Host "Starting Rust Core Web Server (Cargo Run)..." -ForegroundColor Yellow
try {
    cargo run --package forgepress-core
} finally {
    # 4. Clean up background job when terminating
    Write-Host "`nStopping background Vite services..." -ForegroundColor Yellow
    Stop-Job $ViteJob
    Remove-Job $ViteJob
}