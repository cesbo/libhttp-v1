<?php
define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
if (URL_PATH == '/post' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    $content = file_get_contents('php://input');
    for($i=0; $i<50; ++$i) {
        echo $content;
    }
    exit;
}