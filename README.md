# The Image Handler - imhdlr

## To be able to build
##### explicitly specifies the path to the PHP executable:
`export PHP=$(which php)`
#### explicitly specifies the path to the php-config executable:
`export PHP_CONFIG=$(which php-config)`

# Linux 
php -d extension=target/release/libimhdlr.so ./php-examples/test.php
# MacOs
php -d extension=target/release/libimhdlr.dylib ./php-examples/test.php
# Windows
php -d extension=target/release/libimhdlr.dll ./php-examples/test.php

```
