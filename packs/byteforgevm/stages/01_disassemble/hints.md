# Hint 1

First decide which source lines become instructions. Only those lines should receive addresses.

# Hint 2

Parse the file into a sequence, then render that sequence with its indices. Loading and displaying are two separate jobs.

# Hint 3

`lines`, `filter`, and `enumerate` provide the sequence. A format width such as `{ip:04}` supplies the zero-padded address.
