$ErrorActionPreference = 'Stop'

$packageName = 'curator-cli'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$executable = Join-Path $toolsDir 'cu.exe'

# Remove the shim
Uninstall-BinFile -Name 'cu'

# Remove the executable
if (Test-Path $executable) {
    Remove-Item $executable -Force
    Write-Host "Curator CLI has been uninstalled successfully!" -ForegroundColor Green
} else {
    Write-Host "Curator CLI executable not found, but package is being removed." -ForegroundColor Yellow
}
