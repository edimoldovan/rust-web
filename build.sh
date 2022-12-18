#!/bin/bash

# echo 'copying javascript files to public folder...';
# cp assets/node_modules/flatpickr/dist/flatpickr.min.js public/js;
# cp assets/js/web-component.js public/js/;
# cp assets/js/is-land.js public/js/;

echo 'building and minifying global css with rust-based postcss...'
cd assets/css/; npx lightningcss --bundle -m --nesting global.css -o ../../static/css/global.min.css; cd ..; cd ..;

# echo 'copying fonts...'
# cp assets/fonts/* public/fonts/


cargo build
