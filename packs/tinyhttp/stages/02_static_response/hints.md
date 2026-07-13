# Hint 1

Build the response from four visible parts: status line, length header, empty line, and body. Write down the exact missing-file response too.

# Hint 2

Separate “find and read the file” from “format the response.” The formatter should receive the bytes whose length it announces.

# Hint 3

Read the file as bytes and use the byte vector's length for `Content-Length`. A missing file selects status 404 and an empty body.
