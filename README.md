# Clean Flash Player 34.0.0.376 ReleaseOnly

[![License: MIT](https://img.shields.io/badge/installer%20source-MIT-blue.svg)](LICENSE)
[![Flash: 34.0.0.376](https://img.shields.io/badge/Flash-34.0.0.376-orange.svg)](https://github.com/darktohka/clean-flash-builds/releases/tag/v1.54)

这是 [Clean Flash Player 安装器](https://gitlab.com/cleanflash/installer) 的社区维护分支，面向仍然依赖 Flash ActiveX、NPAPI、PPAPI 或独立播放器的旧程序。

本分支的重点不是提供 Debug Player，而是避免普通用户误装 Debug OCX 后，在旧式 ActiveX 宿主程序中反复遇到 `Read of address 0000000C` 访问冲突。

> 本项目不是 Adobe、重橙网络或原 Clean Flash 团队的官方发行版。Flash 已停止全球支持，具有已知安全风险；请仅运行可信内容，优先离线或隔离使用。

## 下载

从本仓库的 [Releases](https://github.com/SacredCrab/CleanFlash_Installer/releases) 下载：

- `CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe`
- `SHA256SUMS.txt`

请在运行前核对 SHA-256。曾完成实际稳定性验证的安装包参考 SHA-256 为：

```text
4e28c1a2e982294ede1dc0f04c36611dd6eea5a2bfecfb7a653cb4597bc03034
```

如果 Release 附件的哈希与此不同，它可能是由公开工作流重新构建的等价版本，但不能宣称与上述实测文件字节完全相同。每个 Release 都应以同页 `SHA256SUMS.txt` 为准。

## 这个版本解决了什么

在已诊断案例中，旧程序启动后出现：

```text
Access violation ... in module 'Flash32_34_0_0_330.ocx'.
Read of address 0000000C.
```

把 Flash 更新到 34.0.0.376 后仍然报错，是因为安装的是 **Debug OCX**，而不是单纯因为版本旧。两个 Debug 版本在同一类代码路径中都可能对空指针偏移 `0x0C` 进行读取。

ReleaseOnly 版本采取三层保护：

1. 安装向导不再显示 Debug 版本选择页；
2. 安装逻辑永远不设置 `InstallFlags.DEBUG`；
3. 提取逻辑拒绝任何名称含 `-debug` 的组件。

同时保留原安装器的重要功能：

- 安装前关闭正在加载 Flash 的相关进程；
- 删除旧 Flash 更新任务和服务；
- 删除 Flash Center、相关服务、缓存、快捷方式和标准安装目录残留；
- 清理旧 Flash 注册表项；
- 安装所选 Release 组件并重新注册 ActiveX；
- 安装独立卸载程序；
- 不安装 Flash Center，不主动加入广告组件。

详细根因、文件识别方法和排查边界见 [故障排查文档](docs/TROUBLESHOOTING.zh-CN.md)。

## 可安装组件

| 组件 | 用途 | 备注 |
| --- | --- | --- |
| ActiveX | Internet Explorer 内核及旧式桌面宿主 | 本次故障对应的组件 |
| NPAPI | Pale Moon、Waterfox Classic 等兼容浏览器 | 现代 Firefox 已不支持 |
| PPAPI | 旧版 Chromium 系浏览器 | 现代 Chrome/Edge 已不支持 |
| Standalone Projector | 直接运行本地 SWF | 建议只打开可信文件 |

64 位 Windows 会同时安装所选组件的 32 位和 64 位文件；Windows 7 及更早系统会自动选择 legacy ActiveX 文件。

## 安装方法

1. 备份需要运行的程序和 SWF 数据。
2. 退出使用 Flash 的程序和浏览器。
3. 以管理员身份运行安装器。
4. 勾选所需组件。运行传统桌面程序通常至少需要 ActiveX。
5. 安装器会先完整清理标准位置中的旧 Flash/Flash Center，再安装 34.0.0.376 Release。
6. 安装完成后重新启动目标程序。
7. 如需确认实际注册的 OCX，可运行：

   ```powershell
   powershell -ExecutionPolicy Bypass -File .\scripts\Get-FlashDiagnostics.ps1
   ```

## 已验证与未验证范围

已验证：

- Windows 10 22H2 64 位；
- 32 位 ActiveX 宿主；
- `Flash32_34_0_0_376.ocx` Release；
- 目标程序连续运行约 4 小时未再次出现原访问冲突。

这不是对所有 Flash 故障的保证。它不能自动修复目标程序自身缺陷、损坏 SWF、服务器下线、证书/网络错误、操作系统损坏，也不一定能清理放在自定义目录中的便携版插件。

## 构建

源码不提交 Adobe/Flash 二进制。构建脚本会从
[darktohka/clean-flash-builds v1.54](https://github.com/darktohka/clean-flash-builds/releases/tag/v1.54)
下载经过校验的 Windows 34.0.0.376 组件，然后生成安装包。

构建说明见 [BUILD.zh-CN.md](docs/BUILD.zh-CN.md)。GitHub Actions 工作流在 Windows runner 上执行同一脚本。

仓库 Fork、Actions 权限、首次 Release 和后续维护步骤见
[PUBLISHING.zh-CN.md](docs/PUBLISHING.zh-CN.md)。

`rust/` 目录来自上游后续实验性移植，本次 34.0.0.376 ReleaseOnly 发布不使用该目录；
当前发布与验证均以原有 C#/.NET Framework 安装器为准。

## 上游与致谢

本项目是在以下工作基础上的维护分支：

- 原始项目与主要作者：[Clean Flash / installer](https://gitlab.com/cleanflash/installer)，作者 darktohka / FlashPatch Team；
- GitHub 备份镜像：[TCOTC/CleanFlash_Installer](https://github.com/TCOTC/CleanFlash_Installer)；
- 34.0.0.376 清理版二进制：[darktohka/clean-flash-builds](https://github.com/darktohka/clean-flash-builds)；
- 二进制补丁工具：[darktohka/FlashPatch](https://github.com/darktohka/FlashPatch)。

完整归属与二进制许可边界见 [NOTICE.md](NOTICE.md)。

## 许可证

安装器源码沿用原项目的 [MIT License](LICENSE)。原版权声明必须保留。

Flash Player 二进制、Adobe 名称和商标不因存放于安装包中而变为 MIT 许可内容；它们仍受各自权利人的条款约束。维护者不授予这些第三方内容的额外权利。
