[CmdletBinding()]
Param(
  [Parameter(Mandatory=$true)]
  [ValidateSet("windows-2019", "ubuntu-18.04", "macos-10.15")]
  [string]$Runner
)

function Write-Output
{
    param ( [string]$name, [string]$value )

    Write-Host ("::set-output name=$name::$value")
}

New-Item -Path . -Name "output" -ItemType "directory"
$artifactName = ""

switch($Runner)
{
  ubuntu-18.04
  {
    cargo install cargo-deb
    cargo deb --manifest-path=cli/Cargo.toml --output=output
    $artifactName = ls output
    Write-Output "artifact_name" $artifactName
    Write-Output "content_type" "application/vnd.debian.binary-package"
  }

  windows-2019
  {
    cargo build --bin esc --release
    Move-Item -Path (Join-Path "target" (Join-Path "release" "esc.exe")) (Join-Path "output" "esc.exe")
    Push-Location output
    $artifactName = "esc-windows-x64.zip"
    Write-Output "artifact_name" $artifactName
    Write-Output "content_type" "application/zip"
    Compress-Archive -Path "esc.exe" -DestinationPath $artifactName
    Pop-Location
  }

  macos-10.15
  {
    cargo build --bin esc --release
    Move-Item -Path (Join-Path "target" (Join-Path "release" "esc")) (Join-Path "output" "esc")
    Push-Location output
    $artifactName = "esc-macos-64bits.tar.gz"
    Write-Output "artifact_name" $artifactName
    Write-Output "content_type" "application/gzip"
    tar -czf $artifactName esc
    Pop-Location
  }
}
