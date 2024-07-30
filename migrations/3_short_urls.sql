CREATE TABLE `short_urls`(
    `id` bigint NOT NULL AUTO_INCREMENT COMMENT '链接id',
    `app_id` bigint NOT NULL COMMENT '应用id',
    `origin_url` varchar(500) NOT NULL COMMENT '源链接',
    `short_key` varchar(8) NOT NULL COMMENT '短链接key',
    `hits` bigint NOT NULL COMMENT '访问量',
    `create_at` int NOT NULL COMMENT '创建时间',
    `update_at` int NOT NULL COMMENT '更新时间',
    PRIMARY KEY (`id`),

    UNIQUE INDEX `ux_short_key`(`short_key`) USING HASH COMMENT '短链接key索引',
    INDEX `idx_app_id`(`app_id`) USING BTREE COMMENT '应用id索引',
    INDEX `idx_origin_url`(`origin_url`) USING HASH COMMENT '源链接索引'
)