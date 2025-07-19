Add todo tasks with:
	rtodo -p high -a Fix linear probing logic
	rtodo -p low -a add comments to text_extractor.

Deletion:
	rtodo -d linear probing logic
	If this is similar to the actual string, it gets removed. Will have to define a threshold.
	If not then, return item not found.

Modification:
	rtodo -m linear probing logic
	If this is similar to the actual string, ask for new string to replace.
	If not then, return item not found.

Make high priority tasks red, low priority white and medium priority yellow.
Critical priority will have a larger font.
