/// Embedded registry resources, matching the original C# resource strings.

pub const UNINSTALL_REGISTRY: &str = r#"[HKEY_LOCAL_MACHINE\Software\Microsoft\Internet Explorer\MAIN\FeatureControl\FEATURE_BROWSER_EMULATION]
"FlashHelperService.exe"=-

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Control Panel\Extended Properties\System.ControlPanel.Category]
"${SYSTEM_64_PATH}\\FlashPlayerCPLApp.cpl"=-

[-HKEY_CURRENT_USER\Software\FlashCenter]
[-HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\App Paths\FlashCenter.exe]
[-HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Uninstall\FlashCenter]
[-HKEY_LOCAL_MACHINE\Software\Classes\AppID\{119DA84B-E3DB-4D47-A8DD-7FF6D5804689}]
[-HKEY_LOCAL_MACHINE\Software\Classes\AppID\{B9020634-CE8F-4F09-9FBC-D108A73A4676}]
[-HKEY_LOCAL_MACHINE\Software\Classes\TypeLib\{37EF68ED-16D3-4191-86BF-AB731D75AAB7}]
[-HKEY_LOCAL_MACHINE\System\ControlSet001\services\Flash Helper Service]
[-HKEY_LOCAL_MACHINE\System\ControlSet001\services\FlashCenterService]
[-HKEY_LOCAL_MACHINE\System\CurrentControlSet\services\Flash Helper Service]
[-HKEY_LOCAL_MACHINE\System\CurrentControlSet\services\FlashCenterService]

[-HKEY_LOCAL_MACHINE\Software\Classes\MacromediaFlashPaper.MacromediaFlashPaper]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Internet Explorer\Low Rights\ElevationPolicy\{FAF199D2-BFA7-4394-A4DE-044A08E59B32}]

[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerUpdateService.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil32_${VERSION_PATH}_ActiveX.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil64_${VERSION_PATH}_ActiveX.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil32_${VERSION_PATH}_Plugin.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil64_${VERSION_PATH}_Plugin.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerPlugin_${VERSION_PATH}.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil32_${VERSION_PATH}_pepper.exe]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashUtil64_${VERSION_PATH}_pepper.exe]

[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Internet Explorer\Low Rights\ElevationPolicy\{FAF199D2-BFA7-4394-A4DE-044A08E59B32}]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Adobe Flash Player ActiveX]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Adobe Flash Player NPAPI]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Adobe Flash Player PPAPI]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Uninstall\Clean Flash Player]

[-HKEY_LOCAL_MACHINE\Software\Classes\.mfp]
[-HKEY_LOCAL_MACHINE\Software\Classes\.sol]
[-HKEY_LOCAL_MACHINE\Software\Classes\.sor]
[-HKEY_LOCAL_MACHINE\Software\Classes\.spl]
[-HKEY_LOCAL_MACHINE\Software\Classes\.swf]
[-HKEY_LOCAL_MACHINE\Software\Classes\AppID\{B9020634-CE8F-4F09-9FBC-D108A73A4676}]
[-HKEY_LOCAL_MACHINE\Software\Classes\CLSID\{B019E3BF-E7E5-453C-A2E4-D2C18CA0866F}]
[-HKEY_LOCAL_MACHINE\Software\Classes\CLSID\{D27CDB6E-AE6D-11cf-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\FlashFactory.FlashFactory]
[-HKEY_LOCAL_MACHINE\Software\Classes\FlashFactory.FlashFactory.1]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{299817DA-1FAC-4CE2-8F48-A108237013BD}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{307F64C0-621D-4D56-BBC6-91EFC13CE40D}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{57A0E747-3863-4D20-A811-950C84F1DB9B}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{86230738-D762-4C50-A2DE-A753E5B1686F}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{D27CDB6C-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Interface\{D27CDB6D-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\MIME\Database\Content Type\application/futuresplash]
[-HKEY_LOCAL_MACHINE\Software\Classes\MIME\Database\Content Type\application/x-shockwave-flash]

[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.1]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.2]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.3]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.4]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.5]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.6]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.7]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.8]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.9]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.10]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.11]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.12]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.13]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.14]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.15]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.16]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.17]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.18]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.19]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.20]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.21]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.22]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.23]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.24]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.25]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.26]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.27]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.28]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.29]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.30]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.31]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.32]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.33]
[-HKEY_LOCAL_MACHINE\Software\Classes\ShockwaveFlash.ShockwaveFlash.34]

[-HKEY_LOCAL_MACHINE\Software\Classes\TypeLib\{57A0E746-3863-4D20-A811-950C84F1DB9B}]
[-HKEY_LOCAL_MACHINE\Software\Classes\TypeLib\{D27CDB6B-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\TypeLib\{FAB3E735-69C7-453B-A446-B6823C6DF1C9}]

[-HKEY_LOCAL_MACHINE\Software\Macromedia]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Internet Explorer\ActiveX Compatibility\{D27CDB6E-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Internet Explorer\ActiveX Compatibility\{D27CDB70-AE6D-11cf-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Microsoft\Internet Explorer\NavigatorPluginsList\Shockwave Flash]

[-HKEY_LOCAL_MACHINE\Software\MozillaPlugins\@adobe.com/FlashPlayer]

[-HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\miniconfig]
[-HKEY_USERS\.DEFAULT\Software\Microsoft\Windows\CurrentVersion\miniconfig]"#;

pub const UNINSTALL_REGISTRY_64: &str = r#"[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Internet Explorer\MAIN\FeatureControl\FEATURE_BROWSER_EMULATION]
"FlashHelperService.exe"=-

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Control Panel\Extended Properties\System.ControlPanel.Category]
"${SYSTEM_32_PATH}\\FlashPlayerCPLApp.cpl"=-

[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\CLSID\{B019E3BF-E7E5-453C-A2E4-D2C18CA0866F}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\CLSID\{D27CDB6E-AE6D-11cf-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\CLSID\{D27CDB70-AE6D-11cf-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{299817DA-1FAC-4CE2-8F48-A108237013BD}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{307F64C0-621D-4D56-BBC6-91EFC13CE40D}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{57A0E747-3863-4D20-A811-950C84F1DB9B}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{86230738-D762-4C50-A2DE-A753E5B1686F}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{D27CDB6C-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Classes\Wow6432Node\Interface\{D27CDB6D-AE6D-11CF-96B8-444553540000}]

[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Macromedia]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Internet Explorer\ActiveX Compatibility\{D27CDB6E-AE6D-11CF-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Internet Explorer\ActiveX Compatibility\{D27CDB70-AE6D-11cf-96B8-444553540000}]
[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Internet Explorer\NavigatorPluginsList\Shockwave Flash]

[-HKEY_LOCAL_MACHINE\Software\Wow6432Node\MozillaPlugins\@adobe.com/FlashPlayer]"#;

pub const INSTALL_GENERAL: &str = r#"[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Control Panel\Extended Properties\System.ControlPanel.Category]
"${SYSTEM_32_PATH}\\FlashPlayerCPLApp.cpl"=dword:0000000a

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerApp.exe]
"DisableExceptionChainValidation"=dword:00000000

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Uninstall\Clean Flash Player]
"DisplayName"="Clean Flash Player ${VERSION}"
"HelpLink"="https://gitlab.com/cleanflash/installer#clean-flash-player"
"NoModify"=dword:00000001
"NoRepair"=dword:00000001
"URLInfoAbout"="https://gitlab.com/cleanflash/installer#clean-flash-player"
"URLUpdateInfo"="https://gitlab.com/cleanflash/installer#clean-flash-player"
"VersionMajor"=dword:00000022
"VersionMinor"=dword:00000000
"Publisher"="CleanFlash Team"
"EstimatedSize"=dword:00011cb8
"DisplayIcon"="${PROGRAM_FLASH_32_PATH}\\FlashUtil_Uninstall.exe"
"UninstallString"="${PROGRAM_FLASH_32_PATH}\\FlashUtil_Uninstall.exe"
"DisplayVersion"="${VERSION}""#;

pub const INSTALL_GENERAL_64: &str = r#"[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows\CurrentVersion\Control Panel\Extended Properties\System.ControlPanel.Category]
"${SYSTEM_32_PATH}\\FlashPlayerCPLApp.cpl"=dword:0000000a

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerApp.exe]
"DisableExceptionChainValidation"=dword:00000000"#;

pub const INSTALL_NP: &str = r#"[HKEY_LOCAL_MACHINE\Software\Macromedia\FlashPlayerPlugin]
"isPartner"=dword:00000001
"Version"="${VERSION}"
"PlayerPath"="${FLASH_64_PATH}\\NPSWF${ARCH}_${VERSION_PATH}.dll"
"UninstallerPath"=-
"isScriptDebugger"=dword:00000000
"isESR"=dword:00000000
"isMSI"=dword:00000000

[HKEY_LOCAL_MACHINE\Software\Macromedia\FlashPlayerPluginReleaseType]
"Release"=dword:00000001

[HKEY_LOCAL_MACHINE\Software\MozillaPlugins\@adobe.com/FlashPlayer]
"Vendor"="Adobe"
"ProductName"="Adobe® Flash® Player ${VERSION} Plugin"
"Path"="${FLASH_64_PATH}\\NPSWF${ARCH}_${VERSION_PATH}.dll"
"Version"="${VERSION}"
"Description"="Adobe® Flash® Player ${VERSION} Plugin"

[HKEY_LOCAL_MACHINE\Software\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerPlugin_${VERSION_PATH}.exe]
"DisableExceptionChainValidation"=dword:00000000"#;

pub const INSTALL_NP_64: &str = r#"[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Macromedia\FlashPlayerPlugin]
"PlayerPath"="${FLASH_32_PATH}\\NPSWF_${VERSION_PATH}.dll"
"Version"="${VERSION}"
"UninstallerPath"=-
"isScriptDebugger"=dword:00000000
"isESR"=dword:00000000
"isMSI"=dword:00000000
"isPartner"=dword:00000001

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Macromedia\FlashPlayerPluginReleaseType]
"Release"=dword:00000001

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\MozillaPlugins\@adobe.com/FlashPlayer]
"ProductName"="Adobe® Flash® Player ${VERSION} Plugin"
"Description"="Adobe® Flash® Player ${VERSION} Plugin"
"Version"="${VERSION}"
"XPTPath"="${FLASH_32_PATH}\\flashplayer.xpt"
"Vendor"="Adobe"
"Path"="${FLASH_32_PATH}\\NPSWF32_${VERSION_PATH}.dll"

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Microsoft\Windows NT\CurrentVersion\Image File Execution Options\FlashPlayerPlugin_${VERSION_PATH}.exe]
"DisableExceptionChainValidation"=dword:00000000"#;

pub const INSTALL_PP: &str = r#"[HKEY_LOCAL_MACHINE\Software\Macromedia\FlashPlayerPepper]
"UninstallerPath"=-
"PlayerPath"="${FLASH_64_PATH}\\pepflashplayer${ARCH}_${VERSION_PATH}.dll"
"isScriptDebugger"=dword:00000000
"isESR"=dword:00000000
"isMSI"=dword:00000000
"isPartner"=dword:00000001
"Version"="${VERSION}"

[HKEY_LOCAL_MACHINE\Software\Macromedia\FlashPlayerPepperReleaseType]
"Release"=dword:00000001"#;

pub const INSTALL_PP_64: &str = r#"[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Macromedia\FlashPlayerPepper]
"UninstallerPath"=-
"PlayerPath"="${FLASH_32_PATH}\\pepflashplayer32_${VERSION_PATH}.dll"
"isScriptDebugger"=dword:00000000
"isESR"=dword:00000000
"isMSI"=dword:00000000
"isPartner"=dword:00000001
"Version"="${VERSION}"

[HKEY_LOCAL_MACHINE\Software\Wow6432Node\Macromedia\FlashPlayerPepperReleaseType]
"Release"=dword:00000001"#;
