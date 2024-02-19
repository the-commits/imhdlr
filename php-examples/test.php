<?php

var_dump(count(imhdlr_get('tests/images')));
var_dump(imhdlr_crop('tests/images', 600, 350, true));
var_dump(imhdlr_squeeze('tests/images', 750, 750, true));
var_dump(imhdlr_squeeze_and_crop('tests/images', 400, 400, true));
