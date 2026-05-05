$ErrorActionPreference = 'Stop'

$packageName = 'raven-rag'
$url64 = "https://github.com/egkristi/ravenrustrag/releases/download/v${env:chocolateyPackageVersion}/raven-windows-amd64.exe"
$checksum64 = '${SHA256_WINDOWS_AMD64}'
$checksumType64 = 'sha256'

$toolsDir = "$(Split-Path -Parent $MyInvocation.MyCommand.Definition)"
$exePath = Join-Path $toolsDir 'raven.exe'

Get-ChocolateyWebFile -PackageName $packageName `
  -FileFullPath $exePath `
  -Url64bit $url64 `
  -Checksum64 $checksum64 `
  -ChecksumType64 $checksumType64

Install-BinFile -Name 'raven' -Path $exePath
