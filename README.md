# WGPU Tutor

这是书的源码哦，成品在[这儿](https://switefaster.github.io/wgpu-tutor)

## 简介

《某不正经的WGPU教程》，又名《小学生坐在马桶上都能看懂的WGPU教程》~~、《高中生站在马桶上也能看懂的WGPU教程》~~。是由[switefaster](https://github.com/switefaster)编写的中文WGPU教程，语言随意，建议读作茶余饭后打发时间用。

## 关于这个仓库

### 工具

本书使用[mdBook v0.4.31](https://github.com/rust-lang/mdBook)编写

> 本书使用了[mdbook-katex](https://github.com/lzanini/mdbook-katex)预处理器，但是后者暂未支持MSVC工具链。若有自行构建需求者，请注意安装。

**注意：** 本书由于[mdBook的缺陷](https://github.com/rust-lang/mdBook/issues/1081)暂不支持中文内容搜索。我已做出[修复方案](https://github.com/rust-lang/mdBook/issues/1081#issuecomment-1621169175)但是目前无人表态。在我的提议被采用或者别的修复方案被采用之前，在搜索框输入中文是无效的。

### 自定义HTML脚标

本书自定义了一些HTML脚标以方便一些样式的书写。
样式定义位于[extra-style.css](./extra-style.css)

- `<mask>黑幕</mask>` 生成一个鼠标移上才会显示的黑幕
- 暂无更多

## 书名参考

- 《某科学的超电磁炮》与《魔法禁书目录》
- 《小学生坐在马桶上都可以看懂的C语言编程入门书》
- 《希灵帝国》（或许？）
