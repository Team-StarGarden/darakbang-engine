DROP INDEX `lemma_index` ON `word`;
DROP INDEX `category_index` ON `word`;
DROP INDEX `position_index` ON `word`;
DROP INDEX `sense_type_index` ON `word`;
DROP INDEX `space_index` ON `word`;

ALTER TABLE `word`
CHANGE `lemma` `pyo_je_eo` TEXT NOT NULL;