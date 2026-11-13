# Rustlings 简体中文版 网站

[Rustlings 简体中文版](https://github.com/SandmeyerX/rustlings-zh-cn) 的在线练习网站，使用 [Astro](https://astro.build/) + [Starlight](https://starlight.astro.build/zh-cn/) 构建。

**在线访问：<https://rustlings.freedev.app/>**

[![Built with Starlight](https://astro.badg.es/v2/built-with-starlight/tiny.svg)](https://starlight.astro.build)

## 简介

本仓库用于自动生成练习页面和构建网站，由 Rust 生成器读取练习文件并转换成 Starlight 页面。想本地练习？试试 [SandmeyerX/rustlings-zh-cn](https://github.com/SandmeyerX/rustlings-zh-cn)

## 功能

- 按 rustlings 的练习分组自动生成页面，每个练习包含：
  - 「题目」和「答案」两个 Tab；
  - 可折叠的提示；
  - `test` / `check` / `clippy` 要求标记徽章；
  - 练习文件与答案文件的路径标题。
- 代码块行号显示与全屏查看。
- 练习与《Rust 程序设计语言》章节对应表。
- 基于 GitHub Actions 的自动构建与 FTPS 增量部署。

## 技术栈

| 部分 | 技术 |
| --- | --- |
| 站点框架 | Astro 7 + Starlight |
| 练习页面生成器 | Rust（`src-builder/`） |
| 代码块增强 | Expressive Code、行号插件、全屏插件 |
| 部署 | GitHub Actions + FTPS（InfinityFree） |

## 目录结构

```text
.
├─ src/                      # 站点源码
│  ├─ assets/                # 图片资源
│  ├─ content/docs/          # Starlight 文档
│  │  ├─ index.mdx           # 首页
│  │  ├─ guides/             # 使用指南
│  │  ├─ reference/          # 参考
│  │  └─ exercises/          # 练习页面（自动生成）
│  └─ content.config.ts
├─ src-builder/              # Rust 练习页面生成器
│  ├─ main.rs          
│  ├─ exercise.rs            # 解析练习元数据与源码
│  ├─ builder.rs             # 生成 MDX
│  ├─ template.mdx           # 分组页面模板
│  └─ ex_template.mdx        # 单个练习模板
├─ public/                   # 静态资源
├─ astro.config.mjs          # Starlight 配置
├─ ec.config.mjs             # Expressive Code 配置
├─ Cargo.toml
└─ package.json
```

## 本地开发

### 前置条件

- Node.js 22 或更高版本
- Rust stable，推荐 edition 2024，Rust 1.85+
- 克隆 [rustlings-zh-cn](https://github.com/SandmeyerX/rustlings-zh-cn) 到本地，建议与本仓库放在同一父目录下

```bash
git clone https://github.com/SandmeyerX/rustlings-zh-cn-website.git --depth 1
```

推荐目录布局：

```text
parent/
├─ rustlings-zh-cn/
└─ rustlings-zh-cn-website/
```

### 安装依赖

```bash
cd rustlings-zh-cn-website
npm ci
```

### 生成练习页面

生成器默认从 `../rustlings-zh-cn/` 读取练习文件，构建结果默认输出到 `build/`。支持通过环境变量配置：

#### 环境变量

| 变量            | 默认值                | 说明                                                         |
| --------------- | --------------------- | ------------------------------------------------------------ |
| `RUSTLINGS_DIR` | `../rustlings-zh-cn/` | rustlings 仓库根目录，可写绝对路径或相对当前工作目录的路径   |
| `BUILD_DIR`     | `build/`              | 生成的 MDX 输出目录；本地开发建议设为 `src/content/docs/exercises` |

**PowerShell：**

```powershell
$env:RUSTLINGS_DIR = "D:\path\to\rustlings-zh-cn"
$env:BUILD_DIR = "src/content/docs/exercises"
cargo run
```

**macOS / Linux：**

```bash
RUSTLINGS_DIR=/path/to/rustlings-zh-cn \
BUILD_DIR=src/content/docs/exercises \
cargo run
```

### 启动开发服务器

```bash
npm run dev
```

修改 rustlings 内容后，重新运行生成器即可；Starlight 会自动刷新页面。

### 生产构建

```bash
npm run build
npm run preview
```

构建结果位于 `dist/`。

### 测试

```bash
cargo fmt --check
cargo test
```

## 内容维护

| 想修改的内容 | 文件 |
| --- | --- |
| 首页 | `src/content/docs/index.mdx` |
| 入门指南 | `src/content/docs/guides/get-started.mdx` |
| 练习与 Rust Book 对应表 | `src/content/docs/reference/rust-book-map.mdx` |
| 404 页面 | `src/content/docs/404.mdx` |
| 导航、侧边栏、语言 | `astro.config.mjs` |
| 代码块相关插件配置等 | `ec.config.mjs` |
| 练习页面的结构 | `src-builder/template.mdx`、`src-builder/ex_template.mdx` |

目前暂不支持英文语言。

## 部署

部署由 [`.github/workflows/deploy-infinityfree.yml`](.github/workflows/deploy-infinityfree.yml) 完成，目前只支持手动触发。

### 触发部署

```bash
# 只查看将要变更的文件，不上传
gh workflow run deploy-infinityfree.yml -f dry_run=true

# 真实部署
gh workflow run deploy-infinityfree.yml -f dry_run=false
```

### Secrets

| 名称 | 说明 |
| --- | --- |
| `FTP_HOST` | InfinityFree FTP 主机 |
| `FTP_USERNAME` | FTP 用户名 |
| `FTP_PASSWORD` | FTP 密码 |
| `FTP_DIR` | 远端目录，必须以 `/` 结尾，例如 `/rustlings.freedev.app/htdocs/` |
