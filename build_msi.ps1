Write-Host "VERSIONS"
Write-Host "================="
rustc --version
cargo --version
Write-Host "npm $(npm --version)"
Write-Host "nodejs $(node --version)"
Write-Host "-----------------"

Write-Host "Building All Projects"
cargo build --workspace --release

Write-Host "Move to $(Get-Location)/gui"
$previous = Get-Location
Set-Location gui

Write-Host "Get Npm deps"
npm i

Write-Host "Building Msi"
npm run tauri build -- --release

Write-Host "Move back to $($previous)"
Set-Location $previous
