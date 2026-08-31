# Prediction

Building the index does everything tokenizing did, then groups the occurrences and
sorts them. Before measuring, predict what fraction of the total time the grouping and
sorting will add on top of tokenizing: a rounding error, roughly a third, or more than
the tokenizing itself.

Also predict how that fraction would move if the corpus doubled in size while its
vocabulary stayed about the same.
