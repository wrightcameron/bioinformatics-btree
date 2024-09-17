#!/bin/sh

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BUILD_DIR=$SCRIPT_DIR/../target/debug
GBK_FILES=$SCRIPT_DIR/../data/geneBankFiles

case $# in
0) echo "Usage: " `basename $0` " <datafile name (from the data/files_gbk folder)> "; exit 1;;
esac

datafile=$1
for i in 1 2 3 4 5 6 7 8 9 10 20 31
do
	time $BUILD_DIR/gene-bank-create-btree --cache=1 --degree=0 --gbkfile=$GBK_FILES/$datafile --length=$i --cachesize=5000 --debug=1
	mv dump $datafile.dump.$i
done

