<?php
$fd = fopen("log.txt", 'w') or die("не удалось создать файл");
define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
fputs($fd, dt()." - start connection \n");
if (URL_PATH == '/post' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    $content = file_get_contents('php://input');
    ob_implicit_flush(1);
    header('Transfer-Encoding: chunked');
    http_response_code(200);
    fputs($fd, dt()." - 200 code \n");
    for($i = 0; $i < 100; $i++) {
       printf("%x\r\n%s\r\n", strlen($content), $content);
       ob_flush();
       usleep(100000);
    }
    fputs($fd, dt()." - good end connection \n");
    printf("0\r\n\r\n");
    exit;
}
fputs($fd, dt()." - bad end connection \n");

function dt() {
    return date("Y-m-d H:i:s");
}
