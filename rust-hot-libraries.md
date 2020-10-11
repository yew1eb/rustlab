# Rust常用库

## Cli

### Clap  
Building CLI tools in Rust is a match made in heaven — check out ripgrep and Rust’s own Cargo. Hyperfast start up time, small binary size, type safe code, runtime safe binary, cross compiling to almost every architecture you might want.  
To start building CLI tools, clap is a fantastic CLI library, it’s so good I don’t see any reason to have a dozen alternatives like common in other languages. In fact, if you want to try Rust, I would recommend taking a look at rustup and then trying out this library and see where it gets you.
  
使用说明：https://rustcc.cn/article?id=921ad2c0-09af-4271-ae62-4b21ce281a2b  
examples: https://github.com/clap-rs/clap/tree/master/examples  


### actix web
Rust 异步Web 框架 Actix-web（包括 Actor 框架 Actix）  
https://www.rectcircle.cn/posts/rust-actix/  


### Tokio
Tokio是一个事件驱动的非阻塞I / O平台，用于使用Rust编程语言编写异步应用程序。在较高的层面上，它提供了一些主要组件：

基于多线程，工作窃取的任务调度程序。
由操作系统的事件队列（epoll，kqueue，IOCP等）支持的反应器。
异步TCP和UDP套接字。
这些组件提供构建异步应用程序所需的运行时组件。

https://tokio-zh.github.io/document/  


### serde
Like clap, serde is a feature rich and performant generic serialization library. In fact, thinking about Java and .NET, I don’t remember a serialization library that was this well made from all aspects — ergonomics and performance.  
Don’t try reading/writing from/to files on your own, instead — write your data types first and make serde do all the work. As a bonus you can mix and match the data format (YAML, JSON) after everything is done.   

https://github.com/serde-rs/serde  
 
serde = { version = "1.0.116", features = ["derive"] }  
serde_json = { version = "1.0.33", features = ["raw_value"] }    
serde_derive = 1.0.0

 
### reqwest
Reqwest follows the gold standard for HTTP client libraries like request, superagent and requests and applies it to Rust perfectly. It’s my go-to library for HTTP clients, is feature packed and complete.   
  
### rayon
Rayon is a “data parallelism library for Rust” or in simple words, give it data and it will know how to split it into independent chunks and work all your CPU cores.
Or even more simply, give it a list and it’ll parallelize map over it, amonst other features. Extremely useful for CLI tools; not all languages are good with parallelism over the command line.  


### slog and log
slog is a very complete logging suite for Rust. It is a core followed by many plugins such as term for terminal output, json for JSON output and more.
You should know that there is also log which aims to be part of standard Rust, and is a simpler alternative. I’ve personally switched to log from slog for this reason.   

### itertools
Wouldn’t hurt to have a few more operators over your lists would it? especially as many or most of these are zero cost. With itertools you get that. Great if you’re a fan of libraries like lodash.  


### hyper
hyper is a fast HTTP implementation written in and for Rust (as opposed to those written in C, that cover for performance in dynamic languages). You’ll find hyper to be in almost every high level library that you use, and if you use it directly, it feels a bit like Netty or Finagle. I found myself treating hyper both as an HTTP toolbox (using pieces from it) and as a whole, building a server over it.  

### Actix
Guess what doesn’t use hyper? Actix. Actix tries to be simpler, and from my experience — it delivers. Often I use Actix instead of Hyper because it’s more highlevel and for service purposes — more mature. Today, I default to using Actix directly over Hyper, unless I need to build something low level, or have a library that requires Hyper directly (there’s many).   


### proptest
proptest is a property based testing library for Rust. Ever since I’ve used QuickCheck in my short Haskell stint, I’ve looked for these kind of libraries in every language I’ve used — libraries that propose that they’ll find a failing test case for you automatically by intelligently thinking up a set of input data that trips up your code.  

### Rayon 数据并行计算库  
https://rustcc.cn/article?id=181e0a73-6742-42a9-b7a1-1c00bef436c2  
Rayon 是一个Rust的数据并行计算库。它非常轻巧，可以轻松地将顺序计算转换为并行计算。同时保证不会有数据争用情况出现。

### smol - 异步rumtime
smol是一个轻量而高效的异步runtime。它采用了对标准库进行扩展的方式，整个runtime只有大约1500行代码。作者stjepang大神是大名鼎鼎crossbeam的作者。而他之前参与tokio和async-std的开发的经验和思考，产生出了从头开始构建的smol这个库。实际上在达到和tokio以及async-std相似的性能的前提下，smol代码短线精悍，完全没有依赖mio库，API更加简单，并且没有unsafe代码！而且，它还兼容tokio和async-std。让我们看个简单的例子  
https://rustcc.cn/article?id=2a02d42f-4b27-40f1-ad0e-2015d3413bb7  

### Crossbeam


### Tokio / Futures  
futures01 = { package = "futures", version = "0.1.25" }  
futures = { version = "0.3", default-features = false, features = ["compat", "io-compat"] }  
tokio = { version = "0.2.13", features = ["blocking", "fs", "signal", "io-std", "macros", "rt-core", "rt-threaded", "uds", "sync"] }  
tokio-openssl = "0.4.0"  
tokio-retry = "0.2.0"  
tokio-util = { version = "0.3.1", features = ["codec"] }  
async-trait = "0.1"  

###  anyhow和thiserror - 错误处理库
Anyhow提供了一个anyhow::Error trait（有点类似failure::Error）。而得益于std::error::Error所做的修改，它anyhow::Error是与std::error::Error兼容的。也就是说，对于自定义的错误类型，只需要实现std::error::Error即可。这对于程序的兼容性是一大利好，也因此failure库被日渐废弃。而thiserror正是方便大家为自定义的错误使用宏实现std::error::Error而设计的。  
  https://github.com/dtolnay/thiserror  

### libloading
For those of you who would like to mix Go or any other c-lib library into their Rust frontend, libloading make this simple.
Building medium to large projects with Rust the last year, I accept that some parts of the Rust ecosystem aren’t ready yet, and am not shy of building these in other languages (mostly Go)— only to hook everything back into Rust with libloading.  


### sqlx  
https://github.com/launchbadge/sqlx  

### diesel
A safe, extensible ORM and Query Builder for Rust

https://github.com/diesel-rs/diesel


### kafka client

https://crates.io/crates/kafka – Kafka client  
rdkafka = { version = 0.24.0, features = [libz, ssl, zstd], optional = true }
rdkafka-sys = { path = rdkafka-sys, version = 2.0.0, default-features = false }

#### pulsar
pulsar = { version = 1.0.0, default-features = false, features = [tokio-runtime], optional = true }


### lettre
功能强大的邮件库  
https://rustcc.cn/article?id=a6d08d7d-39f3-4d1c-9938-f3b5c1ba67a2  

### tempfile - 基础实用的临时文件库
https://rustcc.cn/article?id=2a445f70-a511-4e40-81d8-80684af965a1   

### hyper - 底层http库
https://rustcc.cn/article?id=3ec2fda6-b79a-4fb0-ba5e-656bfa1a2cc2

### Rocket - 流行的网络开发框架
https://rustcc.cn/article?id=140a65d6-7fd5-4e7e-b52c-e160369a6cd4  
Rocket是一个基于Rust编写的上层网络框架,是目前rust主流的网络框架之一，有8.8k的star。而它的http部分就是基于之前提到的hyper。按官方说法，具有如下三个特点：1安全无误、开发体验好 2自动解决请求处理的类型问题，且无需全局状态 3各种可插拔的可选组件。那让我们来一起看一看吧～  
  
  
### csv - 文件读写库
https://rustcc.cn/article?id=17166256-989f-4a9c-a27a-440756280c02  


### rand
rand = 0.7

### url
url = 2.1.1  


### regex
regex = 1.3.9

### hash
seahash = { version = 3.0.6, optional = true }
sha-1 = 0.9
sha2 = 0.9
sha3 = 0.9
crc = 1.0.0
hex = 0.4.2
base64 = { version = 0.12.3, optional = true }
uuid = { version = 0.8, features = [serde, v4] }
md-5 = 0.9

### jemallocator
jemallocator = { version = 0.3.0, optional = true }

### leveldb  
leveldb = { version = 0.8, optional = true, default-features = false }


### lz4 zstd
lz4 = { version = 1.23, optional = true }
zstd = { version = 0.5, optional = true }

### lru
lru = 0.4.3

### log
log = 0.4.8
logfmt = { version = 0.0.2, optional = true }

### bytes
bytes = { version = 0.5.6, features = [serde] }
bytesize = { version = 1.0.0, optional = true }

### chrono
chrono = { version = 0.4.6, features = [serde] }
chrono-tz = 0.5.3  


### others
backoff = 0.1.5
bit-vec = 0.6
bloom = 0.3.2
bollard = { version = 0.8.0, optional = true }
cidr-utils = 0.4.2
colored = 1.9
db-key = 0.0.5
derivative = 1.0
derive_is_enum_variant = 0.1.1
env_logger = 0.7.1
evmap = { version = 10.0.2, features = [bytes], optional = true }
exitcode = 1.1.2
flate2 = { version = 1.0, optional = true }
futures = 0.3
futures = 0.3.0
futures-io = 0.3
futures-timer = 3.0
futures_codec = { version = 0.4, optional = true }
getset = 0.1.1
glob = 0.3.0
grok = { version = ~1.0.1, optional = true }
hdrhistogram = 7.0.0
headers = 0.3
hostname = 0.3.1
http = 0.2
indexmap = {version = 1.5.1, features = [serde-1]}
inventory = 0.1
lazy_static = 1.3.0
libc = 0.2.0
listenfd = { version = 0.3.3, optional = true }
maxminddb = { version = 0.14.0, optional = true }
native-tls = 0.2
nom = 5.0.0
nom = { version = 5.1.2 }
notify = 4.0.14
num_cpus = 1.10.0
once_cell = 1.3
pem = 0.8
pest = 2.1.3
pest_derive = 2.1.0
pin-project = 0.4.23
portpicker = 0.1.0
prost = 0.6.0
prost = 0.6.1
prost-build = 0.6.0
prost-derive = 0.6.0
prost-types = 0.6.1
snafu = { version = 0.6, features = [futures-01, futures] }
snap = { version = 1.0, optional = true }
stream-cancel = 0.4.3
string_cache = 0.7.3
strip-ansi-escapes = { version = 0.1.0}
structopt = 0.3.13
syslog = 5
syslog_loose = { version = 0.4.0, optional = true }
task-compat = 0.1
toml = 0.4
typetag = 0.1
warp = { version = 0.2.5, default-features = false, optional = true }
