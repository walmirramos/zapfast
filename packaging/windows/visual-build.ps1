param(
    [Parameter(Mandatory)][string]$Binary,
    [Parameter(Mandatory)][string]$Wix,
    [Parameter(Mandatory)][string]$Iscc,
    [Parameter(Mandatory)][string]$CrtDir,
    [Parameter(Mandatory)][string]$OutputDir
)
$ErrorActionPreference='Stop'
$repo=[IO.Path]::GetFullPath((Join-Path $PSScriptRoot '../..'))
$binaryFile=(Resolve-Path -LiteralPath $Binary).Path
$cli=New-Object System.Diagnostics.ProcessStartInfo
$cli.FileName=$binaryFile
$cli.Arguments='--help'
$cli.UseShellExecute=$false
$cli.CreateNoWindow=$true
$cli.RedirectStandardOutput=$true
$cli.RedirectStandardError=$true
$process=[Diagnostics.Process]::Start($cli)
$help=$process.StandardOutput.ReadToEnd()
$cliError=$process.StandardError.ReadToEnd()
$process.WaitForExit()
if($process.ExitCode -ne 0) {throw "Application executable did not answer --help: $cliError"}
if(($help -join "`n") -match '--demo-shot|--demo-page') {throw 'Do not distribute the demo-enabled QA binary. Build cargo --release without the demo feature.'}
$wixTool=(Resolve-Path -LiteralPath $Wix).Path
$innoTool=(Resolve-Path -LiteralPath $Iscc).Path
$output=(New-Item -ItemType Directory -Force $OutputDir).FullName
$version=[regex]::Match((Get-Content (Join-Path $repo 'Cargo.toml') -Raw),'(?m)^version = "([^"]+)"').Groups[1].Value
if(-not $version) {throw 'Cargo package version is missing'}
$build=(New-Item -ItemType Directory -Force (Join-Path $repo ('target/visual-package/'+[Guid]::NewGuid().ToString('N')))).FullName
$payload=(New-Item -ItemType Directory (Join-Path $build 'payload')).FullName
Copy-Item -LiteralPath $binaryFile -Destination (Join-Path $payload 'zapfast.exe')
foreach($document in 'README.md','LICENSE','THIRD-PARTY-NOTICES.md') {
    Copy-Item -LiteralPath (Join-Path $repo $document) -Destination $payload
}
Copy-Item -LiteralPath (Join-Path $repo 'packaging/WHATSAPP_EMOJI.md') -Destination (Join-Path $payload 'WHATSAPP-EMOJI-NOTICE.md')
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'zapfast-installer.txt') -Destination $payload
$runtime=Get-ChildItem -LiteralPath $CrtDir -File -Filter '*.dll'
if(-not $runtime) {throw 'The app-local x64 MSVC runtime is missing'}
$runtime | Copy-Item -Destination $payload

# Preserve the original MSI's upgrade code, components, account locations,
# shortcuts, running-process check and updater marker. Add only runtime files.
[xml]$installer=Get-Content -LiteralPath (Join-Path $PSScriptRoot 'zapfast.wxs') -Raw
$ns=New-Object Xml.XmlNamespaceManager($installer.NameTable)
$ns.AddNamespace('w','http://wixtoolset.org/schemas/v4/wxs')
$component=$installer.SelectSingleNode('//w:Component[w:File[@Id="ZapFastExe"]]',$ns)
if(-not $component) {throw 'Original MSI executable component not found'}
$documents=$installer.SelectSingleNode('//w:Component[w:File[@Id="Notices"]]',$ns)
if(-not $documents) {throw 'Original MSI documents component not found'}
$notice=$installer.CreateElement('File',$ns.LookupNamespace('w'))
$notice.SetAttribute('Id','WhatsAppEmojiNotice')
$notice.SetAttribute('Name','WHATSAPP-EMOJI-NOTICE.md')
$notice.SetAttribute('Source',(Join-Path $payload 'WHATSAPP-EMOJI-NOTICE.md'))
$notice.SetAttribute('KeyPath','no')
$null=$documents.AppendChild($notice)
foreach($dll in $runtime) {
    $file=$installer.CreateElement('File',$ns.LookupNamespace('w'))
    $file.SetAttribute('Id',('VisualRuntime_'+$dll.Name.Replace('.','_')))
    $file.SetAttribute('Name',$dll.Name)
    $file.SetAttribute('Source',(Join-Path $payload $dll.Name))
    $file.SetAttribute('KeyPath','no')
    $null=$component.AppendChild($file)
}
$generated=Join-Path $build 'visual.wxs'
$installer.Save($generated)
Copy-Item -LiteralPath (Join-Path $PSScriptRoot 'zapfast.ico') -Destination $build
$msi=Join-Path $output "ZapFast-Windows11-Visual-$version-x64.msi"
& $wixTool build $generated -arch x64 -ext WixToolset.Util.wixext -d "Version=$version" -d "SourceDir=$payload" -pdbtype none -o $msi
if($LASTEXITCODE) {throw 'Original Windows Installer build failed'}
& $innoTool "/DPackageFile=$msi" "/DArtifactDir=$output" "/DVersion=$version" (Join-Path $PSScriptRoot 'visual-bootstrap.iss')
if($LASTEXITCODE) {throw 'Executable installer build failed'}

$portable=(New-Item -ItemType Directory (Join-Path $build 'portable')).FullName
Get-ChildItem -LiteralPath $payload -File | Where-Object Name -ne 'zapfast-installer.txt' | Copy-Item -Destination $portable
Copy-Item -LiteralPath (Join-Path $repo 'packaging/zapfast-portable.txt') -Destination $portable
Compress-Archive -Path (Join-Path $portable '*') -DestinationPath (Join-Path $output "ZapFast-Windows11-Visual-$version-Portable-x64.zip") -Force
Get-ChildItem -LiteralPath $output -File | Where-Object Extension -in '.exe','.msi','.zip' | Get-FileHash -Algorithm SHA256 | ForEach-Object {
    "$($_.Hash.ToLower())  $(Split-Path $_.Path -Leaf)"
} | Set-Content (Join-Path $output 'SHA256SUMS.txt')
