-- Your SQL goes here

CREATE TABLE `users_new` (
    `id` INTEGER NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL DEFAULT 'anonymous',
    `hair_color` TEXT,
    `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO `users_new` (`id`, `name`, `hair_color`, `created_at`, `updated_at`)
SELECT `id`, `name`, `hair_color`, `created_at`, `updated_at` FROM `users`;

DROP TABLE `users`;

ALTER TABLE `users_new` RENAME TO `users`;