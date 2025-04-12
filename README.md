rcli

rust command line interface

## csv

### Usage

```sh
rcli csv --input assets/juventus.csv
```

### Options

- `--input`: The input csv file
- `--output`: The output csv file
- `--header`: The header
- `--delimiter`: The delimiter

```
duckdb
select * from read_csv('assets/juventus.csv',auto_detect=true,header=true,delimiter=',')
```
