<?php

define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));

if (URL_PATH == '/get' && $_SERVER['REQUEST_METHOD'] == 'GET') {
    header('Content-Type: text/plain');
    echo 'Hello, world!';
    exit;
}

if (URL_PATH == '/post-length' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    $content = file_get_contents('php://input');
    header('Content-Type: text/plain');
    echo $content;
    exit;
}

if (URL_PATH == '/post-chunked' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    $content = file_get_contents('php://input') . "\r\n";
    ob_implicit_flush(1);
    header('Transfer-Encoding: chunked');
    for($i = 0; $i < 10; $i++) {
       printf("%x\r\n%s\r\n", strlen($content), $content);
       ob_flush();
       usleep(10000);
    }
    printf("0\r\n\r\n");
    exit;
}
