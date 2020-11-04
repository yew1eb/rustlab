# 使用Rust开发完整的REST API



## 使用到的第三方库

## actix-web使用文档
https://www.rectcircle.cn/posts/rust-actix/

actix-web
https://actix.rs/docs/getting-started/


## diescl使用文档
http://diesel.rs/guides/getting-started/

## diesl 与 mysql 编译兼容
https://medium.com/@miikey84/an-ghost-error-for-installing-diesel-cli-on-macox-1982c88c796d
https://medium.com/@kelvintran/rust-diesel-cli-on-window-and-mysqlclient-issue-1cd85bb5d919
[patch.crates-io]
# https://rustcc.cn/article?id=c2daff4a-12a4-42a1-bdbd-7a106789589d
# Try setting MYSQLCLIENT_LIB_DIR to point to the directory where mysqlclient.dll is stored.
#mysqlclient-sys = { git = "https://github.com/pzmarzly/mysqlclient-sys", rev = "acd1b2b" }
mysqlclient-sys = { git = 'https://github.com/sgrif/mysqlclient-sys' }

RUSTFLAGS="-L/usr/local/mysql/lib"  cargo install diesel_cli --no-default-features --features mysql --force

MYSQLCLIENT_LIB_DIR="/usr/local/mysql/lib"

## mysql

root
root


```
CREATE TABLE `user` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `last_name` varchar(100) DEFAULT NULL,
  `first_name` varchar(100) DEFAULT NULL,
  `role_type` tinyint(1) DEFAULT NULL,
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;
```



## 参考资料
+ A web application completely in Rust<https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471>
https://zhuanlan.zhihu.com/p/107049817
https://cprimozic.net/blog/rust-rocket-cloud-run/
https://www.dazhuanlan.com/2020/01/20/5e2504d9bd662
https://doc.xuwenliang.com/docs/rust/1819
https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471
https://doc.xuwenliang.com/docs/rust/2588
https://codeburst.io/how-to-build-a-rest-api-to-execute-system-commands-using-actix-rust-a-step-by-step-guide-e257d5442b16


通过例子学 Rust http://llever.com/rust-by-example-cn/hello/comment.html