A [rangoli](https://en.wikipedia.org/wiki/Rangoli) is beautiful form of art originating in India. The idea of creating a rangoli utilizing letters from the English alphabet came from a Python programming challenge. Below is a sample of the text output. 

```
--------e--------
------e-d-e------
----e-d-c-d-e----
--e-d-c-b-c-d-e--
e-d-c-b-a-b-c-d-e
--e-d-c-b-c-d-e--
----e-d-c-d-e----
------e-d-e------
--------e--------

----------------------l----------------------
--------------------l-k-l--------------------
------------------l-k-j-k-l------------------
----------------l-k-j-i-j-k-l----------------
--------------l-k-j-i-h-i-j-k-l--------------
------------l-k-j-i-h-g-h-i-j-k-l------------
----------l-k-j-i-h-g-f-g-h-i-j-k-l----------
--------l-k-j-i-h-g-f-e-f-g-h-i-j-k-l--------
------l-k-j-i-h-g-f-e-d-e-f-g-h-i-j-k-l------
----l-k-j-i-h-g-f-e-d-c-d-e-f-g-h-i-j-k-l----
--l-k-j-i-h-g-f-e-d-c-b-c-d-e-f-g-h-i-j-k-l--
l-k-j-i-h-g-f-e-d-c-b-a-b-c-d-e-f-g-h-i-j-k-l
--l-k-j-i-h-g-f-e-d-c-b-c-d-e-f-g-h-i-j-k-l--
----l-k-j-i-h-g-f-e-d-c-d-e-f-g-h-i-j-k-l----
------l-k-j-i-h-g-f-e-d-e-f-g-h-i-j-k-l------
--------l-k-j-i-h-g-f-e-f-g-h-i-j-k-l--------
----------l-k-j-i-h-g-f-g-h-i-j-k-l----------
------------l-k-j-i-h-g-h-i-j-k-l------------
--------------l-k-j-i-h-i-j-k-l--------------
----------------l-k-j-i-j-k-l----------------
------------------l-k-j-k-l------------------
--------------------l-k-l--------------------
----------------------l----------------------
```

**Rust Alphabet Rangoli: CLI Version**

At the root level of this repo, we build and execute the workspace using Cargo:

`$>cargo build -p raylib-alpha-rangoli`

`$>cargo run -p raylib-alpha-rangoli -- -n 5` - Generates the rangoli pattern using the first five letters of the alphabet, `a-e`. The text output is the first example shown above.

**Rust Alphabet Rangoli:: Raylib Version**

TBD.