Write-Host "Building All Projects"
cargo build --workspace --release

Write-Host "Building Msi"
$previous = Get-Location
cd gui
npm run tauri build -- --release
cd $previous
