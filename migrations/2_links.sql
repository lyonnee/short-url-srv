CREATE TABLE `links` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `short_key` varchar(255) NOT NULL COMMENT '短链接key',
  `long_url` text NOT NULL COMMENT '原始的长链接',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
  `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `ux_short_key` (`short_key`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;