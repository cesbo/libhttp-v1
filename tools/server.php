<?php
define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
if (URL_PATH == '/post' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    $content = file_get_contents('php://input');
    ob_implicit_flush(1);
    header('Transfer-Encoding: chunked');
    http_response_code(200);
    for($i = 0; $i < 100; $i++) {
       printf("%x\r\n%s\r\n", strlen($content), $content);
       ob_flush();
       usleep(100000);
    }
    printf("0\r\n\r\n");
    exit;
}