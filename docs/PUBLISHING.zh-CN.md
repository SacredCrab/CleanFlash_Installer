# GitHub 发布指南

## 推荐方式：Fork 原 GitHub 镜像

Fork 能自动保留提交历史，并在仓库页显示 `forked from
TCOTC/CleanFlash_Installer`，这是最直观的来源说明。

1. 打开 https://github.com/TCOTC/CleanFlash_Installer/fork
2. Owner 选择自己的账号。
3. 本维护 Fork 当前使用仓库名 `CleanFlash_Installer`。
4. Description 建议填写：

   ```text
   Community-maintained Clean Flash 34.0.0.376 installer that always installs Release builds and preserves full Flash/Flash Center cleanup.
   ```

5. 创建公开 Fork。
6. 将本维护分支的提交推送到默认分支。
7. 在仓库 Settings → Actions → General 中确认工作流具有
   `Read and write permissions`；工作流还在 YAML 中显式声明了
   `contents: write`。
8. 如果 GitHub 默认暂停了 Fork 的 Actions，进入 Actions 页确认启用。

## 首次 Release

第一次推送建议使用普通提交信息，让工作流只生成 Actions Artifact。下载该
Artifact，在真实 Windows 环境完成安装、目标程序、升级和卸载测试后，再触发公开
Release。不要把“工作流构建成功”等同于“运行时已经验证”。

有两种触发方式：

- 推送提交信息含 `[release]` 的提交；
- 在 Actions → `Build ReleaseOnly installer` → `Run workflow` 中把
  `publish_release` 设为 `true`。

工作流成功后应出现：

```text
v34.0.0.376-release-only.1
```

附件应包括：

- `CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe`
- `SHA256SUMS.txt`

## 发布后检查

- 仓库首页明确链接原 GitLab 项目、GitHub 镜像、clean-flash-builds 和
  FlashPatch；
- `LICENSE` 中原版权声明未被删除或替换；
- `NOTICE.md` 明确源码与 Flash 二进制的许可边界；
- Release 页面不是只放 EXE，也包含说明、校验文件和风险提示；
- 仓库中没有第三方目标程序、私人诊断数据、账号信息或专有 SWF；
- Release 的 EXE 哈希与 `SHA256SUMS.txt` 一致；
- Actions 日志显示上游 7z 和 Release OCX 哈希验证通过；
- Issues 已启用，故障模板可以正常创建。

## 建议仓库 Topics

```text
cleanflash
flash
flash-player
activex
legacy-software
windows
release-only
```

## 后续更新版本

不要只修改版本号。每次上游更新至少需要：

1. 记录新的发布页和资产 URL；
2. 独立获取并固定上游资产 SHA-256；
3. 更新所有组件文件名和 Release OCX 预期哈希；
4. 审查 Release/Debug 文件布局是否变化；
5. 更新版本信息、变更记录、Release Notes 与标签；
6. 完成安装、升级、卸载和至少一个真实旧程序测试；
7. 发布新的校验文件，不覆盖历史版本的证据。
