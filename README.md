# Darakbang Engine

설명 추가 예정

# Dependency
## Nightly Rust

이 프로젝트는 [Rocket](https://github.com/SergioBenitez/Rocket/issues/19) 으로 인해 부득이하게 Nightly Rust를 사용하고 있습니다.

## MariaDB Connector

이 프로젝트는 [Diesel](https://diesel.rs) 로 인해 MariaDB C 컨넥터가 필요합니다.

다음 명령어로 설치히세요

```shell script
# Fedora 32 이후
sudo dnf install mariadb-devel

# Ubuntu
sudo apt install mariadb-dev
```