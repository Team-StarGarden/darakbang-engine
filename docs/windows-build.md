# Windows MariaDB Connector

> Windows 10 20H1(2004), Rust Nightly 2020년 3월 22일 버전으로 테스트 하였습니다.

# Install libmysqlclient via Vcpkg

1. [Vcpkg 레포지토리](https://github.com/microsoft/vcpkg)의 readme를 읽고 설치합니다.
2. 환경 변수를 설정합니다.
    * (선택) ```PATH```에 ```vcpkg를 설치한 경로```를 추가합니다.
    * ```VCPKG_ROOT```의 값으로 ```vcpkg를 설치한 경로```로 설정합니다.
    * ```RUSTFLAGS의``` 값으로 ```-Ctarget-feature=+crt-static```로 설정합니다.
    * ```VCPKGRS_DYNAMIC```의 값으로 ```1```로 설정합니다.
3. ```vcpkg install libmysql:x64-windows```를 실행합니다.
    * i7-7700을 기준으로 약 42분 정도 소요됩니다.
4. ```vcpkg intergrate install```을 실행합니다.
    * 관리자 권한이 필요할 수 있습니다.