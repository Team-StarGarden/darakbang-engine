CREATE TABLE `user` (
    `uid`           VARCHAR(255) PRIMARY KEY,
    `service_name`  TEXT NOT NULL,
    `user_name`     TEXT NOT NULL,
    `point`         INT NOT NULL
)