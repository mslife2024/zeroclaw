# ZeroClaw 国际化（i18n）覆盖率和结构

本文档定义了 ZeroClaw 文档的本地化结构，并跟踪当前覆盖率。

最后更新时间：**2026 年 2 月 21 日**。

## 规范布局

使用以下国际化路径：

- 根项目概览：`README.md`（仅英文）
- 完整本地化文档树：`docs/i18n/<语言区域>/...`
- 可选的兼容性垫片位于 docs 根目录：
  - `docs/commands-reference.<语言区域>.md`
  - `docs/config-reference.<语言区域>.md`
  - `docs/troubleshooting.<语言区域>.md`

## 语言区域覆盖率矩阵

| 语言区域 | 根 README | 规范文档中心 | 命令参考 | 配置参考 | 故障排除 | 状态 |
|---|---|---|---|---|---|---|
| `en` | `README.md` | `docs/README.md` | `docs/commands-reference.md` | `docs/config-reference.md` | `docs/troubleshooting.md` | 权威来源 |
| `zh-CN` | `README.md` | `docs/README.md` | - | - | - | `docs/i18n/zh-CN/` 下的部分文章 |
| `ja` | `README.md` | `docs/README.md` | - | - | - | `docs/i18n/ja/` 下的部分文章 |
| `ru` | `README.md` | `docs/README.md` | - | - | - | `docs/i18n/ru/` 下的部分文章 |
| `fr` | `README.md` | `docs/README.md` | - | - | - | `docs/i18n/fr/` 下的部分文章 |
| `vi` | `README.md` | `docs/i18n/vi/README.md` | `docs/i18n/vi/commands-reference.md` | `docs/i18n/vi/config-reference.md` | `docs/i18n/vi/troubleshooting.md` | 完整树本地化 |

## 根 README 完整性

仓库仅维护英文根目录 `README.md`。各语言不再提供单独的根级 README 变体。

## 分类索引国际化

分类目录（`docs/getting-started/`、`docs/reference/`、`docs/operations/`、`docs/security/`、`docs/hardware/`、`docs/contributing/`、`docs/project/`）下的本地化 `README.md` 文件目前仅存在英文和越南文版本。其他语言的分类索引本地化将延后处理。

## 本地化规则

- 技术标识符保持英文：
  - CLI 命令名称
  - 配置键
  - API 路径
  - 特征/类型标识符
- 优先使用简洁的、面向运维的本地化，而非逐字翻译。
- 本地化页面变更时更新"最后更新" / "最后同步"日期。
- 完整目录树的本地化（例如越南文）应在其中心 `README.md` 上保留"其他语言"部分。

## 添加新的语言区域

1. 在 `docs/i18n/<语言区域>/` 下创建规范文档树（完整树时至少包含 `README.md`、`commands-reference.md`、`config-reference.md`、`troubleshooting.md`）。
2. 添加语言区域链接到：
   - 各本地化中心中已有的"其他语言"部分
   - `docs/SUMMARY.md` 中的语言入口部分
3. 可选地添加 docs 根目录垫片文件以保持向后兼容性。
4. 更新此文件（`docs/maintainers/i18n-coverage.md`）并运行链接验证。

## 评审检查清单

- 所有本地化入口文件的链接可解析。
- 没有语言区域引用过时的文件名（例如 `README.vn.md`）。
- 目录（`docs/SUMMARY.md`）和文档中心（`docs/README.md`）与活跃语言区域保持一致。
