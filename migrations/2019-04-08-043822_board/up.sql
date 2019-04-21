CREATE TABLE `boards` (
     board_id CHAR(36) NOT NULL,
     name CHAR(36) NOT NULL,
     short_name CHAR(5) NOT NULL,
     description VARCHAR(255) DEFAULT NULL,
     board_type TINYINT(2) UNSIGNED DEFAULT 0 NOT NULL,
     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
     PRIMARY KEY (`board_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;

ALTER TABLE `threads` ADD COLUMN `board_id` CHAR(36) NOT NULL AFTER `thread_id`;
ALTER TABLE `threads` ADD FOREIGN KEY `threads_ibfk_2`(`board_id`) REFERENCES `boards`(`board_id`) ON DELETE CASCADE;