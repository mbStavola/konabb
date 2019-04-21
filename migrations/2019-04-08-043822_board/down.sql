ALTER TABLE `threads` DROP FOREIGN KEY `threads_ibfk_2`;
ALTER TABLE `threads` DROP COLUMN `board_id`;
DROP TABLE `boards`;