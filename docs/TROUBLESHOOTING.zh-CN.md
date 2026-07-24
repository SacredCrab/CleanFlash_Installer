# `Read of address 0000000C` 故障排查

## 典型现象

旧式 Windows 程序启动或加载 Flash 内容时弹出类似错误：

```text
Access violation at address 78DA7A22 in module
'Flash32_34_0_0_330.ocx'. Read of address 0000000C.
```

更新后也可能看到：

```text
Access violation at address 1FC579E2 in module
'Flash32_34_0_0_376.ocx'. Read of address 0000000C.
```

地址前半部分每次运行可能变化；关键特征是模块名称为
`Flash32_...ocx`，并且读取地址为 `0000000C`。

## 已确认根因

诊断机器注册的是 **32 位 Debug ActiveX OCX**：

| 文件 | 类型 | 大小 | SHA-256 |
| --- | --- | ---: | --- |
| `Flash32_34_0_0_376.ocx` | Debug | 12,770,792 | `11206f2555bb41de8a254a8820a05106d4ae17848d9e57cdace973223a3612e5` |
| `Flash32_34_0_0_376.ocx` | Release | 11,891,688 | `4e1caa0b6805e5d4e4f737c4389b8e94e92ca49e7d7aef17c3773a9d744dcc42` |

对 34.0.0.330 与 34.0.0.376 Debug OCX 的故障位置进行静态比较后，两个版本都会落到同类指令：

```text
mov ebx, [edx+0x0C]
```

故障时 `edx` 为零，因此访问的是空指针偏移 `0x0C`。这解释了为什么只把
Debug Player 从 34.0.0.330 更新到 34.0.0.376 仍然会报错。

## 推荐解决方法

1. 退出目标程序及所有正在使用 Flash 的浏览器。
2. 运行 `CleanFlash_34.0.0.376_ReleaseOnly_Installer.exe`。
3. 至少选择 ActiveX；如果程序使用其他插件接口，再选择相应组件。
4. 允许安装器完成旧 Flash、Flash Center、服务、注册表和标准目录清理。
5. 安装后重新运行诊断脚本：

   ```powershell
   powershell -ExecutionPolicy Bypass -File .\scripts\Get-FlashDiagnostics.ps1
   ```

6. 确认实际注册文件为 `Flash32_34_0_0_376.ocx`，大小及 SHA-256 对应
   Release 文件。

## 为什么普通安装器可能没有解决

原安装器同时提供 Release 与 Debug 选择。Debug 是给 ActionScript/Flash
开发调试使用的内容调试器，不是普通用户的“功能更完整版本”。一旦误选，
更新到更高版本仍可能保留同一类兼容性问题。

ReleaseOnly 分支跳过 Debug 选择、禁止设置 Debug 标志，并拒绝提取
`-debug` 组件，避免再次误装。

## 仍然报错怎么办

先检查报告中的 OCX 哈希：

- 如果仍是 Debug 哈希，说明注册或文件替换没有成功。以管理员身份重新安装，
  并确认安全软件没有阻止注册。
- 如果已经是 Release 哈希，但仍读取 `0000000C`，不要反复覆盖安装。此时可能是
  目标程序自身缺陷、特定 SWF 内容、第三方加载器或另一份自定义目录 OCX。
- 如果模块不是 `Flash32_...ocx`，本页诊断不适用。
- 如果错误来自服务器连接、登录、证书或资源下载，Flash 插件替换通常无效。

提交 Issue 时请附诊断文本，不要上传第三方程序本体。公开前删除个人路径和账号信息。

## 实测边界

在 Windows 10 22H2 64 位、32 位 ActiveX 宿主、34.0.0.376 Release OCX
组合中，目标程序连续运行约四小时未复现原故障。这证明了该具体环境中的修复，
不等于对所有程序和 Windows 版本作出保证。
