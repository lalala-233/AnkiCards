# 英语单词项目

## TODO

- [] 整理英语单词
- - [x] 初中
- - [] 高中
- - [] 大四
- - [] 大六
- - [] 大八
- - [] ~~COCA（工作量爆炸）~~

## 需要做什么？

1. 删除不合理的排版。
2. 删除过长或过短的句子。
3. 删除部分不重要且难背的单词或缩写或地名。
4. 更换排版。
5. 去重（已完成）。

## 检查并生成文件

使用 Rust 进行粗略检查生成文件。

```Rust
cargo r --release
```

如果你需要更改检查的文件，请在 [src/main.rs](src/main.rs) 中更改。
