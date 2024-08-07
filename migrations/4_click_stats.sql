CREATE TABLE `click_stats` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `link_id` bigint NOT NULL COMMENT '链接表的 id',
  `clicked_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '点击时间',
  `referrer` varchar(255) DEFAULT NULL COMMENT '点击来源页面',
  `country` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `link_id` (`link_id`),
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;