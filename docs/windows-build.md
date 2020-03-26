# Windows MariaDB Connector

## MSVC ABI

> ❌ MSVC ABI에서는 현재 지원하지 않습니다. GNU ABI가 필요합니다.

## GNU ABI

> Windows 10 20H1 (2004), Rust 1.42,
> MySQL 5.7, MariaDB 10.4 에서 테스트하였습니다.

1. MySQL 5.7을 설치합니다.
    * 실제 DBMS는 MariaDB 이므로 MySQL 서비스를 등록할 필요가 없습니다.
2. ```MYSQLCLIENT_LIB_DIR``` 환경 변수를 설정합니다.
    * 64비트 환경에서는 기본적으로 ```C:\Program Files\MySQL\MySQL Server 5.7\lib```입니다.
    * ```libmysql.dll```, ```libmysql.lib```, ```mysqlclient.lib``` 세 파일이 있으면 됩니다.
3. 빌드하고 실행해 봅니다.
    * ```cargo run```
4. ```libmysql.dll```을 찾지 못할 때에는 해당 파일을 ```C:\Windows\System32```에 복사 한 후 다시 시도해 봅니다.
