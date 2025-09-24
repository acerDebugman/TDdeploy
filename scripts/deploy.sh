#!/bin/bash

path=/root/zgc/dev/dev_setup/TDengine/debug/build
tpath=/usr/local/taos
echo "before:"
#md5sum $path/taosx $tpath/taosx
#md5sum $path/taos-explorer $tpath/taos-explorer
md5sum $path/bin/taosd $tpath/bin/taosd
md5sum $path/bin/taosadapter $tpath/bin/taosadapter

#systemctl stop taosx
#cp taosx /usr/local/taos/bin/taosx
#systemctl stop taos-explorer
#cp taos-explorer /usr/local/taos/bin/taos-explorer
#cp taosd /usr/local/taos/bin/taosd

echo "deploy taosd..."
systemctl stop taosd
cp $path/bin/taosd $tpath/bin/taosd

systemctl stop taosadapter
cp $path/bin/taosadapter $tpath/bin/taosadapter

echo "deploy taos driver..."
cp $path/lib/libtaosnative.so $tpath/driver/libtaosnative.so.3.3.7.5
cp $path/lib/libtaos.so $tpath/driver/libtaos.so.3.3.7.5

echo "after:"
#md5sum $path/taosx $tpath/taosx
#md5sum $path/taos-explorer $tpath/taos-explorer
md5sum $path/bin/taosd $tpath/bin/taosd
md5sum $path/lib/libtaosnative.so $tpath/driver/libtaosnative.so.3.3.7.5
md5sum $path/lib/libtaos.so $tpath/driver/libtaos.so.3.3.7.5
md5sum $path/bin/taosadapter $tpath/bin/taosadapter

#systemctl start taos-explorer
#systemctl start taosx
systemctl start taosd
systemctl start taosadapter

