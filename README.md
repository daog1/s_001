
# 用python写solana合约

最近把solana写智能合约给重学了一遍,虽然我原来说就专注于evm生态,现在又重拾solana rust有点打脸.人总是在成长.

关于solana写合约的教程很多,好像学会的人挺少,在上一次学solana的时候我觉得我也算会了,本来就有rust的基础嘛,今年真正写写发现不是这么一回事.

账号系统,pda还是有点复杂,solana也算evm生态上的很多东西不一样了,重新学起了,最近觉得小技初成,吹吹牛.

发现一个项目,挺好玩,所以分享分享.
[seahorse](https://www.seahorse.dev/) 用python写solana合约.

大家可以直接按照官方文档玩,但是你会遇到挺多困难.
我别的不吹,解决这些困难.

## 安装seahorse
需要一些前置的要求,安装上anchor,solana sdk,rust 1.77.1
```
cargo install seahorse-dev
```
如果前面的都没问题. 命令行里面能找到 seahorse 
```
seahorse -V
Seahorse 0.2.
```

## 建立项目
我建立了一个
```
seahorse init s_001
```
## build
现在就开始痛苦旅程
### 问题
```
seahorse build
✗ Compiling s_001... (note: if this is your first time building, it might take a few minutes)
Error: anchor build -p s_001 failed:

This is most likely a bug in the Seahorse compiler!

If you want to help the project, you can report this:
  - as a Github issue (https://github.com/solana-developers/seahorse/issues).

Thanks!

[2024-04-07T02:17:44.219197000Z ERROR cargo_build_sbf] Failed to obtain package metadata: `cargo metadata` exited with an error: warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
    note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
    note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
    note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
        Updating crates.io index
    error: failed to select a version for `solana-program`.
        ... required by package `pyth-sdk-solana v0.8.0`
        ... which satisfies dependency `pyth-sdk-solana = "^0.8.0"` 
```
### 搞定
直接修改 src/Cargo.toml
```
solana-program = "=1.17.17"
pyth-sdk-solana = { version = "0.10.1", optional = true }
```
## 再来
### 问题
```
seahorse build
✗ Compiling s_001... (note: if this is your first time building, it might take a few minutes)
Error: anchor build -p s_001 failed:

This is most likely a bug in the Seahorse compiler!

If you want to help the project, you can report this:
  - as a Github issue (https://github.com/solana-developers/seahorse/issues).

Thanks!

error: package `toml_edit v0.21.1` cannot be built because it requires rustc 1.69 or newer, while the currently active rustc version is 1.68.0-dev
Either upgrade to rustc 1.69 or newer, or use
cargo update -p toml_edit@0.21.1 --precise ver
where `ver` is the latest version of `toml_edit` supporting rustc 1.68.0-dev
```
### 搞定

```
cargo update -p toml_edit@0.21.1 --precise 0.21.0
```

### 最小化代码能跑
```
seahorse build

warning: `s_001` (lib) generated 1 warning
    Finished release [optimized] target(s) in 0.33s
warning: the following packages contain code that will be rejected by a future version of Rust: libc v0.2.153
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 3`
```  
这就是成功了
## 运行测试
```
anchor test

1) s_001
       Is initialized!:
     TypeError: program.methods.initialize is not a function
```
查看target/idl/s_001.json
发现,instructions 里面没东西,所有需要写点代码.

## 修改代码
打开s_001.py
```
class Hello(Account):
  bump: u8

@instruction
def init(owner: Signer, hello: Empty[Hello]):
    bump = hello.bump()
    hello = hello.init(
        payer=owner,
        seeds=['hello']
    )
    hello.bump = bump
```
修改tests/s_001.ts
```
const tx = await program.methods.init().rpc();
```
## 再次运行
```
seahorse build
anchor test

Your transaction signature 4U81hwLVMc3qVDvkvTsfApSL1AfmjXKJe23aPtAQvpzkToFNED8Cse7DTYpjEE5RKEHyzTsAPKbmFv4dw5GvyYnB
    ✔ Is initialized! (450ms)
  1 passing (454ms)
```

这就搞定了.

这就开始了python写solana合约的第一步.

我简单读了一下代码s_001和seahorse的代码.
这玩意是通过对python代码进行解析然后生成,anchor rust的代码,通过直接编译rust 合约代码产生合约进行部署.

好了,多看文档和代码吧,我估计使用这个也并不会让你写solana的合约变得简单.

## 一点小东西
如果对anchor有每次加
```
solana-program = "=1.17.17"
```
的困扰,可以使用我的版本.
[anchor 1.17.17定制](https://github.com/daog1/anchor/tree/solana17)