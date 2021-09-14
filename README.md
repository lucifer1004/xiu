# 锈 xiu

Aren't you _疲于_ writing Rust programs in English? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Chinese touch to your
programs?

**锈 (xiu)** (Chinese for _Rust_) is here to save your day, as it allows you to
write Rust programs in Chinese language, using Chinese keywords, Chinese function names.

You don't feel at ease using only Chinese words? Don't worry! Chinese Rust is fully 
compatible with English-Rust, so you can mix both at your convenience.

Here's an example of what can be achieved with 锈 (xiu):

### trait and impl (也即 性 和 阐)

```rust
xiu::锈! {
    外 箱 锈;

    用 中::仓::典 作 典;

    性 键值 {
        函 写(&身, 键: 串, 值: 串);
        函 读(&身, 键: 串) -> 果<或<&串>, 串>;
    }

    静 变 籍: 或<典<串, 串>> = 无;

    构 实;

    阐 键值 为 实 {
        函 写(&身, 键: 串, 值: 串) {
            定 书 = 危 {
                籍.取入(标::准)
            };
            书.入(键, 值);
        }

        函 读(&身, 键: 串) -> 果<或<&串>, 串> {
            若 定 有(书) = 危 { 籍.作引() } {
                好(书.取(&键))
            } 否则 {
                错("未之有也".进())
            }
        }
    }
}
```

### 他例

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.

## 奉

First, _拜谢_ for considering participating in this joke, the
Chinese government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `zhu` (Chinese for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Chinese.

## but why would you do 此

- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.

## 他言

- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)

## 标

Not yet.

## 许

[WTFPL](http://www.wtfpl.net/).
