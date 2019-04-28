<?php
define('URL_PATH', parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH));
if (URL_PATH == '/post' && $_SERVER['REQUEST_METHOD'] == 'POST') {
    for($i=0; $i<100; ++$i) {
        print "test";
        echo(file_get_contents('php://input'));
        ussleep(50);
    }
    exit;
}