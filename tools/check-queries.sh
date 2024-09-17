#!/bin/sh
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
BUILD_DIR=$SCRIPT_DIR/../target/debug
QUERIES_DIR=$SCRIPT_DIR/../data/queries
QUERY_RESULTS_DIR=$SCRIPT_DIR/../results/query-results

case $# in
0) echo "Usage: " `basename $0` " <datafile name (from the data/files_gbk folder)> "; exit 1;;
esac


datafile=$1
for i in 1 2 3 4 5 6 7 8 9 10 20 31
do
	echo "\nRunning queryfile " query$i "on $datafile.btree.data.$i.0\n"
	time -p $BUILD_DIR/gene-bank-search-btree --cache=0 --degree=0 --btreefile=$datafile.btree.data.$i.0 --length=$i --queryfile=$QUERIES_DIR/query$i --debug=0  > $QUERIES_DIR/query$i-$datafile.out
done
echo

for i in 1 2 3 4 5 6 7 8 9 10 20 31
do
	diff -q -w $QUERIES_DIR/query$i-$datafile.out $QUERY_RESULTS_DIR/query$i-$datafile.out
	if test "$?" = "0"; then
		echo "----> Query-Test-$i PASSED!"
	else
		echo "----> Query-Test-$i FAILED@$#!"
	fi

done
echo

