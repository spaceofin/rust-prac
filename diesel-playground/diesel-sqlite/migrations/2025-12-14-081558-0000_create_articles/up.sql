-- Your SQL goes here


CREATE TABLE `articles`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`title` TEXT NOT NULL,
	`body` TEXT NOT NULL,
	`draft` BOOL NOT NULL,
	`publish_at` TIMESTAMP NOT NULL,
	`visit_count` INTEGER NOT NULL
);

