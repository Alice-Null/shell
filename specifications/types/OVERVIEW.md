Short list;
	booleans
	strings
	characters
	integers (signed)
	floats (evil incarnate)
	fractions (like floats but with different tradeoffs)
	bytes (8 bits)
	arrays
	dictionaries (classes but evil)

# Booleans
it's either true or false
this is really simple why do i have a section for this

# Characters
a single utf-8 symbol.
eg; 'A', '🐟', '𓆝 ', '࿔'
some combined characters, eg; '◌⃑', or '◌⃢'
are not a single character

# Strings
it holds a bunch of utf-8 characters
equivalent to a list of characters

# Integers
whole numbers (1, 3, 490)
64 bit signed
MAX: 9223372036854775807
MIN: -9223372036854775808

# Floats
64 bit floating point numbers
I am not explaining how these work under the hood
it's black magic
rust 64 floating point numbers.
probably standardized i don't know
eg; 129.0929
these suck just don't use them please i beg of you
i didn't implement them wrong, i just don't like them
floats are evil and mean boooo
everyone go yell at floats (they're cool just bleh)
they consume integers when added

# Fractions
like floats but they take up more space
nicer but probably slower or something
probably won't make this actually but worth a shot probably
they absorb floats and ints
float + fraction 	= fraction
fraction + float	= fraction
int + fraction		= fraction
fraction + int		= fraction

# bytes
basically just simplest form of data
8 bits
represents arbitrary data
usually present in arrays

# Arrays
speaking of arrays
they're expandable
for exact details see rust's Vec documentation

# dictionaries
treated like a class but mutable
named fields of arbitrary values
that's about it
