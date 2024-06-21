-- Add migration script here
CREATE TABLE `users`  (
  `id` bigint NOT NULL COMMENT '用户id',
  `email` varchar(250) COMMENT 'email地址',
  `phone` varchar(14) COMMENT '电话号码',
  `salt` varchar(16) NOT NULL COMMENT '密码盐',
  `ciphertext` varchar(64) NOT NULL COMMENT '密码hash',
  `create_at` int NOT NULL COMMENT '创建时间',
  `update_at` int NOT NULL COMMENT '更新时间',
  PRIMARY KEY (`id`),

  UNIQUE INDEX `ux_email`(`email`) USING HASH COMMENT '邮箱索引',
  UNIQUE INDEX `ux_phone`(`phone`) USING HASH COMMENT '手机号索引'
);