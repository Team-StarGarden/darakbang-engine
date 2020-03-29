ALTER TABLE `word`
CHANGE `pyo_je_eo` `lemma` VARCHAR(255) NOT NULL;

CREATE FULLTEXT INDEX `lemma_index` ON `word` (`lemma`);
CREATE INDEX `category_index` ON `word` (`category`);
CREATE INDEX `position_index` ON `word` (`position`);
CREATE INDEX `sense_type_index` ON `word` (`sense_type`);
CREATE INDEX `space_index` ON `word` (`space`);