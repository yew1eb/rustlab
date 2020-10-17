# 使用Rust开发完整的REST API

https://zhuanlan.zhihu.com/p/107049817
https://cprimozic.net/blog/rust-rocket-cloud-run/

## 使用到的第三方库
actix-web


## mysql

```
CREATE TABLE `user` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
  `last_name` varchar(100) DEFAULT NULL,
  `first_name` varchar(100) DEFAULT NULL,
  `role_type` tinyint(1) DEFAULT NULL,
  `create_time` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8
```



## 参考资料
+ A web application completely in Rust<https://medium.com/@saschagrunert/a-web-application-completely-in-rust-6f6bdb6c4471>