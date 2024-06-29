# NGram Analysis

This is a tool to analyze the n-grams of a raw text corpus. It can be used to analyze the n-grams of a text file and generate a report of the n-grams found.
I use this tool to get data on bigrams to optimise keyboard layouts, when doing it on raw text, I strip all punctuation and special characters from the text file with the following command:

```bash
sed 's/[^a-zA-Z ]//g' "text.txt" | tr 'A-Z' 'a-z' | sed -E 's/[[:space:]]+/ /g' >> text-clean.txt
```


## Example data

There is some example data in the `./data` directory. You can use this data to test the tool. This is a small excerpt from the [Wikipedia Corpus off of corpusdata.org](https://www.corpusdata.org/formats.asp). 
