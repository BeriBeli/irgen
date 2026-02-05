param(
  [switch]$SkipTests,
  [switch]$SkipNsis,
  [switch]$SkipMsi,
  [switch]$SkipZip,
  [switch]$SkipSign,
  [string]$Configuration = "release",
  [string]$ProductName = "irgen",
  [string]$Manufacturer = "BeriBeli",
  [string]$ExeName = "irgen-gui.exe"
)

$ErrorActionPreference = "Stop"

function Require-Command {
  param([string]$Name, [string]$InstallHint)
  if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
    Write-Host "Missing required command: $Name"
    if ($InstallHint) { Write-Host $InstallHint }
    throw "$Name not found"
  }
}

function Get-CargoTargetDir {
  $json = & cargo metadata --format-version 1 --no-deps | Out-String
  if (-not $json) { throw "Failed to read cargo metadata" }
  $data = $json | ConvertFrom-Json
  return $data.target_directory
}

function Get-AppVersion {
  try {
    $version = & python -c "import tomllib; print(tomllib.load(open('Cargo.toml','rb'))['package']['version'])"
    if ($LASTEXITCODE -eq 0 -and $version) { return $version.Trim() }
  } catch {}

  $content = Get-Content -Raw Cargo.toml
  $m = [regex]::Match($content, '^\s*version\s*=\s*"([^"]+)"', 'Multiline')
  if ($m.Success) { return $m.Groups[1].Value }
  throw "Failed to determine version from Cargo.toml"
}

function To-MsiVersion([string]$Version) {
  $parts = $Version.Split('.')
  while ($parts.Count -lt 3) { $parts += '0' }
  return "$($parts[0]).$($parts[1]).$($parts[2])"
}

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
Push-Location $repoRoot
try {
  Write-Host "==> irgen Windows release script"

  Require-Command cargo "Install Rust from https://rustup.rs/ and ensure cargo is on PATH."

  if (-not $SkipTests) {
    Write-Host "==> Running tests"
    & cargo test --locked
  }

  Write-Host "==> Building $ExeName ($Configuration)"
  $buildArgs = @("build", "--locked", "--bin", "irgen-gui")
  if ($Configuration -eq "release") {
    $buildArgs += "--release"
  } else {
    $buildArgs += @("--profile", $Configuration)
  }
  & cargo @buildArgs

  $targetDir = Get-CargoTargetDir
  $exePath = Join-Path $targetDir "$Configuration\$ExeName"

  if (-not (Test-Path $exePath)) {
    Write-Host "==> $ExeName not found at $exePath. Searching under $targetDir..."
    $found = Get-ChildItem $targetDir -Recurse -Filter $ExeName -File -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($found) { $exePath = $found.FullName }
  }

  if (-not (Test-Path $exePath)) {
    throw "$ExeName not found under $targetDir"
  }

  New-Item -ItemType Directory -Force -Path dist | Out-Null
  Copy-Item $exePath dist\$ExeName -Force

  if (-not (Test-Path "dist\$ExeName")) {
    throw "dist\$ExeName not found after copy"
  }

  $appVersion = Get-AppVersion
  $msiVersion = To-MsiVersion $appVersion
  Write-Host "==> Version: $appVersion (MSI: $msiVersion)"

  if (-not $SkipNsis) {
    Require-Command makensis "Install NSIS (e.g. choco install nsis -y) and ensure makensis is on PATH."
    $iconPath = Join-Path $PWD "resources\windows\app-icon.ico"
    Write-Host "==> Building NSIS installer"
    & makensis /DVERSION=$appVersion /DICON_PATH="$iconPath" /DAPP_EXE_PATH="$exePath" resources\windows\installer.nsi
  }

  if (-not $SkipMsi) {
    Require-Command candle.exe "Install WiX Toolset (e.g. choco install wixtoolset -y) and ensure candle.exe is on PATH."
    Require-Command light.exe "Install WiX Toolset (e.g. choco install wixtoolset -y) and ensure light.exe is on PATH."
    Write-Host "==> Building MSI"
    & candle.exe resources\windows\installer.wxs `
      -dVersion=$msiVersion `
      -dProductName=$ProductName `
      -dManufacturer=$Manufacturer `
      -dExeName=$ExeName `
      -out dist\irgen-gui.wixobj
    & light.exe dist\irgen-gui.wixobj -ext WixUIExtension -out dist\irgen-gui-windows.msi
  }

  if (-not $SkipSign) {
    $pfxPath = $env:WINDOWS_CERT_PFX_PATH
    if (-not $pfxPath -and $env:WINDOWS_CERT_PFX) {
      $pfxPath = Join-Path $env:TEMP "signing.pfx"
      [IO.File]::WriteAllBytes($pfxPath, [Convert]::FromBase64String($env:WINDOWS_CERT_PFX))
    }
    if ($pfxPath -and (Test-Path $pfxPath) -and $env:WINDOWS_CERT_PASSWORD) {
      $signtool = (Get-Command signtool.exe -ErrorAction SilentlyContinue).Source
      if (-not $signtool) {
        $signtool = Get-ChildItem "C:\Program Files (x86)\Windows Kits\10\bin" -Recurse -Filter signtool.exe -ErrorAction SilentlyContinue |
          Sort-Object FullName -Descending | Select-Object -First 1 -ExpandProperty FullName
      }
      if ($signtool) {
        $timestamp = "http://timestamp.digicert.com"
        $files = @("dist\$ExeName", "dist\irgen-gui-windows-setup.exe", "dist\irgen-gui-windows.msi")
        foreach ($file in $files) {
          if (Test-Path $file) {
            Write-Host "==> Signing $file"
            & $signtool sign /fd SHA256 /td SHA256 /tr $timestamp /f $pfxPath /p $env:WINDOWS_CERT_PASSWORD $file
          }
        }
      } else {
        Write-Host "signtool.exe not found; skipping signing."
      }
    } else {
      Write-Host "Signing env vars not set; skipping signing."
    }
  }

  if (-not $SkipZip) {
    Write-Host "==> Zipping exe"
    Compress-Archive -Path dist\$ExeName -DestinationPath dist\irgen-gui-windows.zip -Force
  }

  Write-Host "==> Done. Artifacts are in dist\\"
} finally {
  Pop-Location
}
