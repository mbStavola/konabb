CREATE TABLE `posts` (
     post_id CHAR(36) NOT NULL,
     thread_id CHAR(36) NOT NULL,
     user_id CHAR(36) DEFAULT NULL,
     raw_text TEXT NOT NULL,
     rendered_text TEXT DEFAULT NULL,
     created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
     updated_at DATETIME DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
     PRIMARY KEY (`post_id`),
     KEY `thread_id` (`thread_id`),
     KEY `user_id` (`user_id`),
     CONSTRAINT `posts_ibfk_1` FOREIGN KEY (`thread_id`) REFERENCES `threads` (`thread_id`) ON DELETE CASCADE,
     CONSTRAINT `posts_ibfk_2` FOREIGN KEY (`user_id`) REFERENCES `users` (`user_id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;