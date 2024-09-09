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