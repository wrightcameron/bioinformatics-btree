#!/bin/bash
# Import Gene Bank Dump files for test0.gbk and test5.gbk to sqlite3 database.
# Database and table gene_sequence will be created if they don't exist.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
DUMP_FILE_DIR=$SCRIPT_DIR/../results/dumpfiles
GENE_BANK_DATABASE=$SCRIPT_DIR/../data/GeneBankDatabase.db

# Create Database first, so columns are named
sqlite3 $GENE_BANK_DATABASE "CREATE TABLE IF NOT EXISTS gene_sequence(sequence TEXT, frequency INT)"

# sqlite doesn't let you pass in strdin with -, but with '|cat -' we can.  Stoping the need to make perminiate csv files to import
sed 's/ /,/g' $DUMP_FILE_DIR/test0.gbk.dump.1 | sqlite3 $GENE_BANK_DATABASE ".import --csv '|cat -' gene_sequence";
sed 's/ /,/g' $DUMP_FILE_DIR/test0.gbk.dump.5 | sqlite3 $GENE_BANK_DATABASE ".import --csv '|cat -' gene_sequence";
