

```shell
# initialize the project
# TODO
declare-rs init --currency-from <currency> --currency-to <currency> --csv-file <path> --tax <percent> 

# add a new transaction
declare-rs add --date <date> --amount <amount> --currency-from <currency> --currency-to <currency> --date <date> --exchange-rate <rate>

# show all transactions
declare-rs show  --output <format>

declare-rs open

# show currency exchange rate
declare-rs exchange --currency-from <currency> --currency-to <currency> --amount <amount> --date <date>

```

