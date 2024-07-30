CREATE TABLE `apps` (
    `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'app_id',
    `user_id` bigint NOT NULL COMMENT '用户id',
    `app_name` varchar(20) NOT NULL COMMENT '应用名',
    `create_at` int NOT NULL COMMENT '创建时间',
    `update_at` int NOT NULL COMMENT '更新时间',
    PRIMARY KEY(`id`),

    INDEX `idx_user_id` (`user_id`) USING BTREE COMMENT '用户id索引'
)