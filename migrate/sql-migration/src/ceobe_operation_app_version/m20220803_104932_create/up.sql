CREATE TABLE `ceobe_operation_app_version`(
    `id` INTEGER AUTO_INCREMENT PRIMARY KEY NOT NULL,
    `version` VARCHAR(11) NOT NULL UNIQUE,
    `force` BOOLEAN NOT NULL DEFAULT 0,
    `last_force_version` VARCHAR(11) NOT NULL,
    `description` TEXT NOT NULL,
    `create_at` DATETIME NOT NULL DEFAULT NOW(),
    `modify_at` DATETIME NOT NULL DEFAULT NOW(),
    `delete_at` DATETIME NOT NULL DEFAULT '1970-01-01 00:00:00'
)ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci