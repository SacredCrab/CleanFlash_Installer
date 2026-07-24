# 构建 34.0.0.376 ReleaseOnly 安装包

## 原则

- 仓库只保存 MIT 许可的安装器源码、脚本和文档；
- Flash 二进制在构建时从公开上游下载；
- 下载后必须先核对 SHA-256；
- ReleaseOnly 代码不能设置 Debug 标志，也不能提取 `-debug` 目录；
- 发布物放在 GitHub Release，不提交到 Git 历史。

## 固定上游

| 项目 | 值 |
| --- | --- |
| 发布页 | `https://github.com/darktohka/clean-flash-builds/releases/tag/v1.54` |
| 文件 | `ChineseFlash-Patched-Win-34.0.0.376.7z` |
| SHA-256 | `19a8d1036110af024dc877ca96a9184835c3e944fb14ca8d509662f00bf1bd31` |

不要只用文件名判断来源。上游重新上传同名文件时，哈希变化应视为需要人工审计的新输入。

## 本地构建环境

- Windows 10/11 或 Windows Server 2022；
- PowerShell 5.1 或更高；
- Visual Studio 2022 Build Tools（包含 MSBuild 组件）；
- .NET SDK 8.x；
- 7-Zip；
- 网络可访问 GitHub Release 与 NuGet。

构建必须使用 Visual Studio 随附的完整 .NET Framework MSBuild，而不是
`dotnet build`。卸载器与安装器是面向 .NET Framework 4.0 的 WinForms
项目，其中包含旧式非字符串 `.resx` 资源；使用现代 `dotnet` SDK 的
跨平台 MSBuild 可能触发 `MSB3822`/`MSB3823`。

在仓库根目录运行：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\Build-ReleaseOnly.ps1
```

输出：

```text
dist\CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe
dist\SHA256SUMS.txt
```

脚本将依次：

1. 下载并校验上游 7z；
2. 编译 34.0.0.376 卸载器；
3. 只整理 Release ActiveX、NPAPI、PPAPI 和 Standalone 文件；
4. 生成嵌入式 `cleanflash.7z`；
5. 编译并合并安装器依赖；
6. 验证嵌入包不含 Debug 文件；
7. 验证 32 位 Release OCX 哈希；
8. 生成安装包 SHA-256。

## GitHub Actions

`.github/workflows/build-release.yml` 在以下情况运行：

- 手动运行；
- 修改安装器、公共库、卸载器、构建脚本或工作流后推送到默认分支。

该工作流始终只生成 Actions Artifact，不直接公开 Release。真实 Windows 测试通过后，
使用 `.github/workflows/promote-tested-release.yml` 输入原构建 run ID、实测安装包
SHA-256 和源码提交 SHA，把同一个 Artifact 提升为 GitHub Release。提升流程会在公开前
两次核对 EXE 与 `SHA256SUMS.txt`。

## 关于可重复构建

构建脚本关闭 7z 文件时间和多线程压缩元数据，以减少不确定性，但 .NET/ILRepack、
Windows runner 和 NuGet 包的变化仍可能导致最终 EXE 哈希不同。

因此：

- 每个发布必须附自己的 `SHA256SUMS.txt`；
- 只有哈希完全相同时，才能称为与已实测参考包字节相同；
- 功能等价的新构建仍应重新完成安装、卸载和目标程序稳定性测试。
