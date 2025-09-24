#!/bin/bash

home=/app/TDengine
#sudo ln -s $home/debug/build/lib/libtaos.so /usr/lib/libtaos.so
#sudo ln -s $home/debug/build/lib/libtaosnative.so /usr/lib/libtaosnative.so
#sudo ln -s $home/debug/build/lib/libtaosws.so /usr/lib/libtaosws.so

ln -s $home/debug/build/lib/libtaos.so /usr/lib/libtaos.so
ln -s $home/debug/build/lib/libtaosnative.so /usr/lib/libtaosnative.so
ln -s $home/debug/build/lib/libtaosws.so /usr/lib/libtaosws.so

