ALTER TABLE `word`
ADD `group_code`        INT NOT NULL,
ADD `group_order`       INT NOT NULL,
ADD `word_unit`         VARCHAR(16) NOT NULL,
ADD `word_type`         VARCHAR(32) NOT NULL,
ADD `category`          VARCHAR(32),
ADD `definition`        TEXT NOT NULL,
ADD `position`          VARCHAR(32),
ADD `sense_type`        VARCHAR(16) NOT NULL,
ADD `space`             VARCHAR(16) NOT NULL;

CREATE TABLE `region_word` (
    `id`        INT PRIMARY KEY AUTO_INCREMENT,
    `word_id`   INT NOT NULL REFERENCES `word`(`id`),
    `region`    VARCHAR(32) NOT NULL
);

CREATE TABLE `original_language_word` (
    `id`        INT PRIMARY KEY AUTO_INCREMENT,
    `word_id`   INT NOT NULL REFERENCES `word`(`id`),
    `language`  VARCHAR(32) NOT NULL,
    `original`  TEXT NOT NULL
);