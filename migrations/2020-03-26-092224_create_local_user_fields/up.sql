ALTER TABLE `user`
ADD (
    `password` BINARY(32),
    `salt` CHAR(8)
)