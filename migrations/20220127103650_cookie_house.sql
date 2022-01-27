-- Add migration script here
-- 根基表
CREATE TABLE IF NOT EXISTS mansion(
    `id` BIGINT PRIMARY KEY NOT NULL UNIQUE AUTO_INCREMENT,
    `create_time` DATETIME NOT NULL CURRENT_TIMESTAMP,
    `edit_time` DATETIME NOT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
-- 每天预测
CREATE TABLE IF NOT EXISTS each_mansion(
    `id` BIGINT PRIMARY KEY NOT NULL UNIQUE AUTO_INCREMENT,
    `mid` BIGINT NOT NULL,
    -- 预测天
    `date` DATE NOT NULL UNIQUE,
    -- 预测内容动态（可能为空）
    `content` VCHAR(512) DEFAULT NULL,
    -- 约束区
    FOREIGN KEY(`mid`) REFERENCES mansion(id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;
CREATE TABLE IF NOT EXISTS mansion_info(
    `id` BIGINT PRIMARY KEY NOT NULL UNIQUE AUTO_INCREMENT,
    `eid` BIGINT NOT NULL,
    `predict_level` ENUM (`false`, `unknown`, `true`) NOT NULL,
    `info` VCHAR(512) NOT NULL,
    FOREIGN KEY (`eid`) REFERENCES each_mansion(id)
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4;