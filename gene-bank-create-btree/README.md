# Gene Bank Create BTree

## Usage

```bash
./gene-bank-create-btree --cache=<0|1>  --degree=<btree-degree> --gbkfile=<gbk-file> --length=<sequence-length> [--cachesize=<n>] [--debug=0|1]
```

**Note that the arguments can be provided in any order.**

If the name of the GeneBank file is `xyz.gbk`, the subsequence length is `<k>` and the B-Tree degree
is `<t>`, then the name of the B-Tree file should be `xyz.gbk.btree.data.<k>.<t>`

- `<cache>` specifies whether the program should use cache (value `1`) or
no cache (value `0`); if the value is `1`, the `<cache-size>` has to be specified

- `<degree>` is the degree to be used for the B-Tree. If the user specifies `0`, then our
program should choose the optimum degree based on a disk block size of `4096` bytes and the
size of our B-Tree node on disk

- `<gbk-file>` is the input `*.gbk` file containing the input DNA sequences

- `<sequence-length>` is an integer that must be between `1` and `31` (inclusive)

- `[<cache-size>]` is an integer between `100` and `10000` (inclusive) that represents the
maximum number of `BTreeNode` objects that can be stored in memory

- `[<debug-level>]` is an optional argument with a default value of zero

    - `0`: Any diagnostic messages, help and status messages must be printed on standard
    error stream

    - `1`: The program writes a text file named `dump`, containing the frequency and the DNA
    string (corresponding to the key stored) in an inorder traversal, and has the following
    line format:

### Usage Examples

`cargo run -- -g ../data/geneBankFiles/test0.gbk`

`cargo run -- --cache=1 --degree=0 --gbkfile=../data/geneBankFiles/test0.gbk --length=2 --cachesize=5000 --debug=1`

## Benchmark

| gbk file | degree | sequence length | cache | cache size | cache hit rate | run time  |
| -------- | ------ | --------------- | ----- | ---------- | -------------- | --------  |
| test5.gbk|  102   |     20          |  no   |    0       |      0%        |  17.778s  |
| test5.gbk|  102   |     20          |  yes  |    100     |      n/a%      |  22.990s  |
| test5.gbk|  102   |     20          |  yes  |    500     |      n/a%      |  23.243s  |
| test5.gbk|  102   |     20          |  yes  |    1000    |      n/a%      |  22.893s  |
| test5.gbk|  102   |     20          |  yes  |    5000    |      n/a%      |  22.331s  |
| test5.gbk|  102   |     20          |  yes  |    10000   |      n/a%      |  22.991s  |

```bash
time ./target/release/gene-bank-create-btree --cache=0 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=0 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=100 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=100 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=500 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=1000 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=5000 --debug=0
time ./target/release/gene-bank-create-btree --cache=1 --degree=102 --gbkfile=./data/geneBankFiles/test5.gbk --length=10 --cachesize=10000 --debug=0
```
