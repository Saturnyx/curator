$ErrorActionPreference = 'Stop'

$packageName = 'curator-cli'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/Saturnyx/curator/releases/download/v0.3.5/cu-x86_64-pc-windows-msvc.exe'

$packageArgs = @{
  packageName   = $packageName
  unzipLocation = $toolsDir
  fileType      = 'exe'
  url64bit      = $url64
  softwareName  = 'Curator CLI*'
  checksum64    = ''
  checksumType64= 'sha256'
  silentArgs    = ''
  validExitCodes= @(0)
}

# Download and install the executable
$downloadedFile = Get-ChocolateyWebFile @packageArgs

# Rename the downloaded file to cu.exe and place it in tools directory
$targetPath = Join-Path $toolsDir 'cu.exe'
if ($downloadedFile) {
    Move-Item $downloadedFile $targetPath -Force
} else {
    # Fallback if the download path is different
    $possiblePaths = @(
        Join-Path $toolsDir 'cu-x86_64-pc-windows-msvc.exe'
        Join-Path $toolsDir 'cu.exe'
    )
    
    foreach ($path in $possiblePaths) {
        if (Test-Path $path) {
            if ($path -ne $targetPath) {
                Move-Item $path $targetPath -Force
            }
            break
        }
    }
}

# Verify the executable was placed correctly
if (-not (Test-Path $targetPath)) {
    throw "Failed to install curator CLI executable"
}

# Create a shim for the executable so it can be called from anywhere
Install-BinFile -Name 'cu' -Path $targetPath

Write-Host "Curator CLI has been installed successfully!" -ForegroundColor Green
Write-Host "You can now use 'cu' command from anywhere in your terminal." -ForegroundColor Yellow
Write-Host "Try 'cu --help' to get started." -ForegroundColor Yellow
