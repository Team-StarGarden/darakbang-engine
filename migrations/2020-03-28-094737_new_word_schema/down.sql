ALTER TABLE `word`
DROP `group_code`,
DROP `group_order`,
DROP `word_unit`,
DROP `word_type`,
DROP `category`,
DROP `definition`,
DROP `position`,
DROP `sense_type`,
DROP `space`;

DROP TABLE `region_word`;
DROP TABLE `original_language_word`;