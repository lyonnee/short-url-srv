CREATE TABLE `users` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT '用户id',
  `email` varchar(250) DEFAULT NULL COMMENT 'email地址',
  `phone` varchar(14) DEFAULT NULL COMMENT '电话号码',
  `salt` varchar(16) NOT NULL COMMENT '密码盐',
  `ciphertext` varchar(64) NOT NULL COMMENT '密码hash',
  `create_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `update_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `ux_email` (`email`) COMMENT '邮箱索引',
  UNIQUE KEY `ux_phone` (`phone`) COMMENT '手机号索引'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;