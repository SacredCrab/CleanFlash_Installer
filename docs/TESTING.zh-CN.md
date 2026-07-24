# 测试清单

## 发布前最低检查

- [ ] 上游 7z SHA-256 与构建文档一致
- [ ] 构建日志中没有 Debug 文件
- [ ] 嵌入 32 位 Release OCX SHA-256 为
      `4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42`
- [ ] 安装器版本信息为 34.0.0.376
- [ ] 默认流程中不存在 Debug 选择页
- [ ] 旧 Flash/Flash Center 清理成功
- [ ] ActiveX 注册成功
- [ ] 卸载器能完整移除安装内容
- [ ] Release 附件与 `SHA256SUMS.txt` 一致

## 已完成环境

| Windows | 架构 | 组件 | 结果 |
| --- | --- | --- | --- |
| Windows 10 22H2 | 64 位系统、32 位宿主 | ActiveX Release | 目标程序约 4 小时未复现原访问冲突 |

## 建议补充矩阵

| Windows | 32 位 | 64 位 | ActiveX | NPAPI | PPAPI | Standalone | 安装/升级/卸载 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Windows 7 | 待测 | 待测 | 待测 legacy | 待测 | 待测 | 待测 | 待测 |
| Windows 10 | 待扩展 | 已部分验证 | 已部分验证 | 待测 | 待测 | 待测 | 待测 |
| Windows 11 | 不适用 | 待测 | 待测 | 待测 | 待测 | 待测 | 待测 |

记录结果时必须包含安装包哈希和实际加载的插件文件哈希。
