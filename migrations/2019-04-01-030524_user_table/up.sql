CREATE TABLE `users` (
   user_id CHAR(36) NOT NULL,
   username VARCHAR(15) NOT NULL,
   email VARCHAR(40) DEFAULT NULL,
   password VARCHAR(255) NOT NULL,
   PRIMARY KEY (`user_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_unicode_ci;