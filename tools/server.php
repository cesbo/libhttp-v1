<?php


define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
define('METHOD', $_SERVER['REQUEST_METHOD']);
define('TEST_DATA', 'Hello, world!');


if (URL_PATH == '/get' && METHOD == 'GET') {
    header('Content-Type: text/plain');
    echo TEST_DATA;
    exit;
}


if (URL_PATH == '/post-length' && METHOD == 'POST') {
    $content = file_get_contents('php://input');
    header('Content-Type: text/plain');
    echo $content;
    exit;
}


if (URL_PATH == '/post-chunked' && METHOD == 'POST') {
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


if (URL_PATH == '/get-chunked-lf-only' && METHOD == 'GET') {
    ob_implicit_flush(1);
    header('Transfer-Encoding: chunked');
    $content = TEST_DATA . "\r\n";
    for($i = 0; $i < 10; $i++) {
        printf("%x\n%s\n", strlen($content), $content);
        ob_flush();
        usleep(10000);
    }
    printf("0\n\n");
    exit;
}


if (URL_PATH == '/get-chunked-wo-trailer' && METHOD == 'GET') {
    ob_implicit_flush(1);
    header('Transfer-Encoding: chunked');
    $content = TEST_DATA . "\r\n";
    for($i = 0; $i < 10; $i++) {
        printf("%x\r\n%s\r\n", strlen($content), $content);
        ob_flush();
        usleep(10000);
    }
    printf("0\r\n");
    exit;
}
