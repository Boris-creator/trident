A tiny command-line tool made by me in the process of learning Rust. Maybe it is shit code!

## Community

<a href="https://stand-with-ukraine.pp.ua/"><img src="https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg" width="1700"></a>


## Usage

Navigate into folder with `main.rs` file and run one of the commands

```
cargo run stats
cargo run siren
cargo run trident
```
or, if you're using Windows, build the project, navigate to `trident.exe` file and run one of the commands
```
trident.exe stats
trident.exe siren
trident.exe trident
```
Example output of the first command is
```
The losses of the fascist army on the 299 day of the war amounted to:
+-------+-------------------+-------------+------------------+-----------------+------------------------+-------------------------+------+--------------------+-------------------+-------+-------------+--------+------+
| orcs  | artillery systems | helicopters | warships/cutters | cruise missiles | special military equip | vehicles and fuel tanks | MLRS | AA warfare systems | ATGM/SRBM systems | tanks | UAV systems | planes | AFV  |
+-------+-------------------+-------------+------------------+-----------------+------------------------+-------------------------+------+--------------------+-------------------+-------+-------------+--------+------+
| 98800 | 1953              | 264         | 16               | 653             | 175                    | 4592                    | 410  | 212                | 4                 | 2988  | 1657        | 281    | 5969 |
+-------+-------------------+-------------+------------------+-----------------+------------------------+-------------------------+------+--------------------+-------------------+-------+-------------+--------+------+
```
The `siren` command will print a list of areas in which an air raid alert has been declared.
## Contributing

Pull requests are welcome. 

## License

[MIT](https://choosealicense.com/licenses/mit/)