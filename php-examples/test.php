<?php

var_dump("files counted",count(imhdlr_get_images('/Users/magnusaberg/Bazooka/Repos/WP/dumps/envac/uploads')));
var_dump("skipnames: true", "verbose: false");
imhdlr_resize_images('/Users/magnusaberg/Bazooka/Repos/WP/dumps/envac/uploads', 750, 750, true, false);
var_dump("skipnames: false", "verbose: false");
imhdlr_resize_images('/Users/magnusaberg/Bazooka/Repos/WP/dumps/envac/uploads', 600, 350, false, false);
var_dump("skipnames: false", "verbose: false");
imhdlr_resize_images('/Users/magnusaberg/Bazooka/Repos/WP/dumps/envac/uploads', 500, 500, false, false);
var_dump("skipnames: true", "verbose: true");
imhdlr_resize_images('/Users/magnusaberg/Bazooka/Repos/WP/dumps/envac/uploads', 400, 700, true, true);
