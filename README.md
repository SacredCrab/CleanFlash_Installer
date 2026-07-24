# CleanFlash Installer

这是基于原项目继续整理的维护分支，版本为 **34.0.0.376**。

- 原 GitHub 项目：<https://github.com/TCOTC/CleanFlash_Installer>
- 原 GitLab 项目：<https://gitlab.com/cleanflash/installer>
- 上游 Flash 构建：<https://github.com/darktohka/clean-flash-builds/releases/tag/v1.54>
- 下载：<https://github.com/SacredCrab/CleanFlash_Installer/releases/tag/v34.0.0.376-release-only.1>

## 两个安装包

| 文件 | 大小 | 已知情况 |
| --- | ---: | --- |
| `CleanFlash_34.0.0.376_LegacyBundle_56.6MiB.exe` | 59,362,816 字节 | 原有大包，同时包含 Release 与 Debug 组件。在 Windows 10 22H2 x64 上实际运行约 4 小时，未观察到异常；系统当时注册的是 Release OCX。此前错误目前无法复现，因此不能据此证明它修复了错误。 |
| `CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe` | 18,670,592 字节 | 按当前源码重新构建，只打包 Release 组件；其中的核心 Release Flash 文件与大包内对应文件逐字节一致。尚未完成最终用户环境的端到端运行测试。 |

SHA-256：

```text
4e28c1a2e982294ede1dc0f04c36611dd6eea5a2bfecfb7a653cb4597bc03034  CleanFlash_34.0.0.376_LegacyBundle_56.6MiB.exe
dae566c0f9618558d3bcc1bb600eaf1a93d9bd2a1c179cc0b74df6fb4ef309f4  CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe
```

已知的 Release 版 32 位 ActiveX 文件：

```text
Flash32_34_0_0_376.ocx
SHA-256: 4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42
```

诊断时没有观察到目标程序加载 Flash 模块，因此目前不能确认原访问冲突一定由 Flash 引起，也不能保证任一安装包能修复所有电脑。

测试环境：Windows 10 专业版 22H2，x64，内部版本 19045.5912。

安装器源码沿用原项目的 MIT 许可证与原作者署名。捆绑的第三方二进制文件仍归各自权利人所有。本项目不提供任何担保。
