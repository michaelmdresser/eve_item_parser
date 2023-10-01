# eve_item_parser
Item (and item list) parsing for EVE Online

SDE data has to be periodically downloaded to the `data` folder.

``` sh
cd data
rm invTypes.csv
wget https://www.fuzzwork.co.uk/dump/latest/invTypes.csv
```

## CLI testing

``` sh
echo "Paladin 5" | cargo run
```
