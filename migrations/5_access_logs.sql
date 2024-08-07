CREATE TABLE `access_logs` (
  `id` bigint NOT NULL AUTO_INCREMENT,
  `link_id` bigint NOT NULL COMMENT '链接id',
  `accessed_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP COMMENT '访问时间',
  `user_ip` varchar(255) DEFAULT NULL COMMENT '访问者 IP 地址',
  `user_agent` varchar(255) DEFAULT NULL COMMENT '访问者的浏览器或客户端标识',
  PRIMARY KEY (`id`),
  KEY `idx_link_id` (`link_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;