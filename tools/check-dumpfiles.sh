#!/bin/sh
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DUMPFILES_DIR=$SCRIPT_DIR/../results/dumpfiles

case $# in
0) echo "Usage: " `basename $0` " <datafile name (from the data/files_gbk folder)> "; exit 1;;
esac

datafile=$1

echo
echo "Checking your dump files against sample dump files..."
echo

for i in 1 2 3 4 5 6 7 8 9 10 20 31
do
	echo -n "Test-$i: Comparing $datafile.dump.$i"
	diff --ignore-case -w $datafile.dump.$i $DUMPFILES_DIR/$datafile.dump.$i
	if test "$?" = "0"
	then
		echo "----> Test-$i PASSED!"
	fi
done
echo
