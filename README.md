# Darakbang Engine

설명 추가 예정

# Dependencies
## MariaDB Connector

이 프로젝트는 [Diesel](https://diesel.rs) 로 인해 MariaDB C 커넥터가 필요합니다.

다음 명령어로 설치합니다:

### Linux

```shell script
# RHEL family (include CentOS, Fedora...)
sudo dnf install mariadb-devel

# Debian family (include Ubuntu, Linux Mint...)
sudo apt install mariadb-dev

# Arch Linux Family (include Manjaro, MSYS2... etc)
sudo pacman -S mariadb-libs
```

### macOS with Homebrew

```shell script
brew install mariadb
```

### Windows

[여기](docs/windows-build.md)를 참조하세요.

# Create Database

Darakbang을 구동하기 위해서는 utf8mb4로 생성된 Database가 필요합니다.

```sql
CREATE DATABASE darakbang CHARSET utf8mb4;
```
