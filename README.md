# eve_item_parser
Item (and item list) parsing for EVE Online. Check out the tests in `src/lib.rs`
for an idea of what this can parse. Items are assumed to be line-delimited. A
useful utility for diffing item lists is packaged separately at
[michaelmdresser/eve_item_diff](https://github.com/michaelmdresser/eve_item_diff). A practical application can be
found at the [EVE Item Diff](https://michaelmdresser.com/eve-item-diff.html) site.

SDE data has to be periodically downloaded to the `data` folder.

``` sh
cd data
rm invTypes.csv
wget https://www.fuzzwork.co.uk/dump/latest/invTypes.csv
```

## CLI testing

![](media/parse_cli_screenshot.png)

