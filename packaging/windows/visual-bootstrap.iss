#ifndef PackageFile
  #error Pass /DPackageFile with the original MSI package
#endif
#ifndef ArtifactDir
  #error Pass /DArtifactDir
#endif
#ifndef Version
  #define Version "0.21.1"
#endif
[Setup]
AppId=ZapFast.Windows11.Visual.Bootstrap
AppName=ZapFast
AppVersion={#Version}
AppPublisher=ZapFast Community
CreateAppDir=no
Uninstallable=no
PrivilegesRequired=lowest
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
WizardStyle=modern
DisableWelcomePage=no
OutputDir={#ArtifactDir}
OutputBaseFilename=ZapFast-Windows11-Visual-{#Version}-Setup-x64
SetupIconFile=zapfast.ico
Compression=lzma2
SolidCompression=yes

[Files]
Source: "{#PackageFile}"; DestName: "zapfast.msi"; Flags: dontcopy

[Run]
Filename: "{code:InstalledExecutable}"; Description: "Abrir o ZapFast"; Flags: nowait postinstall skipifsilent; Check: ZapFastInstalled

[Code]
function InstalledExecutable(Param: String): String;
var Folder: String;
begin
  Result := '';
  if RegQueryStringValue(HKCU, 'Software\ZapFast', 'InstallFolder', Folder) then
    Result := AddBackslash(Folder) + 'zapfast.exe';
end;

function ZapFastInstalled: Boolean;
begin
  Result := FileExists(InstalledExecutable(''));
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
var Code: Integer; Arguments: String;
begin
  Result := '';
  ExtractTemporaryFile('zapfast.msi');
  Arguments := '/i "' + ExpandConstant('{tmp}\zapfast.msi') + '" /norestart ';
  if WizardSilent then Arguments := Arguments + '/qn'
  else Arguments := Arguments + '/qb';
  if not Exec(ExpandConstant('{sys}\msiexec.exe'), Arguments, '', SW_HIDE, ewWaitUntilTerminated, Code) then
    Result := 'Não foi possível iniciar a instalação do ZapFast.'
  else if Code = 3010 then NeedsRestart := True
  else if Code <> 0 then
    Result := 'A instalação não foi concluída (código ' + IntToStr(Code) + '). Feche o ZapFast e tente novamente.';
end;
