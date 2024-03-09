<?php

var_dump("files counted",count(imhdlr_get_images('tests/images')));
var_dump("skipnames: true", "verbose: false");
imhdlr_resize_images('tests/images', 750, 750, true, false);
var_dump("skipnames: false", "verbose: false");
imhdlr_resize_images('tests/images', 600, 350, false, false);
var_dump("skipnames: false", "verbose: false");
imhdlr_resize_images('tests/images', 500, 500, false, false);
var_dump("skipnames: true", "verbose: true");
imhdlr_resize_images('tests/images', 400, 700, true, true);
