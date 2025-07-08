# Anki Cards

这是一个存储着 Anki 数据的备份。

## 特点

纯文本存储，使用 Git 进行备份。

## 目录

### [New](/New/)

[New](/New/) 目录下存储的是现在正在使用的一些数据。

数据格式为 [AnkiMaker](https://github.com/lalala-233/AnkiMaker) 可识别的 `toml` 文件，以及生成的可导入的 `txt` 文件。

### [Old](/Old/)

[Old](/Old/) 目录下存储的是一些以前的数据，仅作备份。

## english for 分支

目标：整理英语单词。

我将提供不同阶段的单词，并使用[一个 Rust 程序](/New/Eng/project/src/main.rs)来对单词进行整理。整理完后的单词应通过 [TODO.md](/New/Eng/project/TODO.md) 的检查，并将生成的 result 文件合并会主分支，最后删除该分支。

- [] 整理英语单词
- - [x] 初中
- - [] 高中
- - [] 大四
- - [] 大六
- - [] 大八
- - [] ~~COCA（工作量爆炸）~~

## 做点贡献

欢迎来帮忙或指出错误！

如果你有什么问题，可以打开 issue 或提交 pull 请求。
