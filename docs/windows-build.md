# Windows MariaDB Connector

## MSVC ABI

### 테스트된 환경
* Windows
  * Windows 10 20H2
* Visual Studio Build Tools
  * Visual Studio 2019 Build Tools

### 구성
1. MariaDB를 설치합니다.
2. [vcpkg](https://github.com/Microsoft/vcpkg) 를 설치합니다.
3. ```vcpkg install libmysql:x64-windows```를 실행합니다.
4. 빌드하고 실행해 봅니다.

## GNU ABI

### 테스트된 환경
* Windows
  * Windows 10 20H1 이후 버전
* Rust
  * Rust Stable

### 구성
1. MySQL과 MariaDB를 설치합니다.
    * 실제 DBMS는 MariaDB 이므로 MySQL 서비스를 등록할 필요가 없습니다.
2. ```MYSQLCLIENT_LIB_DIR``` 환경 변수를 설정합니다.
    * 일반적으로 ```C:\Program Files\MySQL\MySQL Server 5.7\lib```입니다.
    * ```libmysql.dll```, ```libmysql.lib```, ```mysqlclient.lib``` 세 파일이 있으면 됩니다.
3. ```MYSQLCLIENT_LIB_DIR``` 의 값을 ```PATH```에도 추가합니다.
4. 빌드하고 실행해 봅니다.
