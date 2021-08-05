# WGPU Tutor

这是书的源码哦，成品在[这儿](https://switefaster.github.io/wgpu-tutor)

## 简介

《某不正经的WGPU教程》，又名《小学生站在马桶上都能看懂的WGPU教程》~~、《高中生站在马桶上也能看懂的WGPU教程》~~。是由[switefaster](https://github.com/switefaster)编写的中文WGPU教程，语言随意，建议读作茶余饭后打发时间用。

## 关于这个仓库

### 工具

本书使用[mdBook v0.4.12](https://github.com/rust-lang/mdBook)编写

> 本书使用了[mdbook-linkcheck](https://github.com/Michael-F-Bryan/mdbook-linkcheck)后端，预计使用[mdbook-katex](https://github.com/lzanini/mdbook-katex)预处理器，但是其暂未支持MSVC工具链，目前以原生Mathjax替代。若有自行构建需求者，请注意安装。

### 自定义HTML脚标

本书自定义了一些HTML脚标以方便一些样式的书写。
样式定义位于[extra-style.css](./extra-style.css)，[tag-replacer.js](./tag-replacer.js)会替换相应的tag为`<span class=""></span>`的形式。

- `<mask>黑幕</mask>` 生成一个鼠标移上才会显示的黑幕
- 暂无更多
