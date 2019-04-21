CREATE TABLE `threads` (
     thread_id CHAR(36) NOT NULL,
     user_id CHAR(36) DEFAULT NULL,
     title VARCHAR(64) NOT NULL,
     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
     PRIMARY KEY (`thread_id`),
     KEY `user_id` (`user_id`),
     CONSTRAINT `threads_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;