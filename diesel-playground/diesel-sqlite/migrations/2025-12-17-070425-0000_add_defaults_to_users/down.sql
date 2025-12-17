-- This file should undo anything in `up.sql`

CREATE TABLE `users_old` (
    `id` INTEGER NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL,
    `hair_color` TEXT,
    `created_at` TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP NOT NULL
);

INSERT INTO `users_old` (`id`, `name`, `hair_color`, `created_at`, `updated_at`)
SELECT `id`, `name`, `hair_color`, `created_at`, `updated_at` FROM `users`;

DROP TABLE `users`;

ALTER TABLE `users_old` RENAME TO `users`;

