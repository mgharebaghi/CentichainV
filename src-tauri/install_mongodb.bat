
@echo off
msiexec /l*v mdbinstall.log /qb /i "%~dp0mongodb.msi" ADDLOCAL=ServerService SHOULD_INSTALL_COMPASS=0
if %ERRORLEVEL% EQU 0 (
    echo MongoDB installation completed successfully > mongodb_install_result.txt
) else (
    echo MongoDB installation failed with error code %ERRORLEVEL% > mongodb_install_result.txt
    type mdbinstall.log >> mongodb_install_result.txt
)
