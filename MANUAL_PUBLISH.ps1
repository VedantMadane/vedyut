# Manual Publishing Script for Windows PowerShell
# Run this after: cargo login YOUR_TOKEN

Write-Host "📦 Publishing Vedyut crates to crates.io..." -ForegroundColor Green
Write-Host ""

# Navigate to rust directory
Set-Location "c:\Projects\open-source\vedyut\rust"

function Publish-Crate {
    param($CrateName)
    
    Write-Host "🚀 Publishing $CrateName..." -ForegroundColor Cyan
    Set-Location $CrateName
    
    try {
        cargo publish
        Write-Host "✅ $CrateName published successfully!" -ForegroundColor Green
    }
    catch {
        Write-Host "⚠️  $CrateName may already be published or failed" -ForegroundColor Yellow
    }
    
    Set-Location ..
    Write-Host "Waiting 15 seconds for crates.io to propagate..." -ForegroundColor Gray
    Start-Sleep -Seconds 15
    Write-Host ""
}

# Publish in dependency order
Publish-Crate "vedyut-lipi"
Publish-Crate "vedyut-sandhi"
Publish-Crate "vedyut-prakriya"
Publish-Crate "vedyut-kosha"
Publish-Crate "vedyut-cheda"
Publish-Crate "vedyut-sanskritify"
Publish-Crate "vedyut-core"

Write-Host "🎉 All crates published!" -ForegroundColor Green
Write-Host "View at: https://crates.io/crates/vedyut-core" -ForegroundColor Cyan

Set-Location "c:\Projects\open-source\vedyut"
