parser or something
basic stuff now, i'll make it more fun later
basic syntax is:

COMMAND2 ARG1 ARG2 ARG3... ARGN; COMMAND1 ARG1 ARG2 ...

or

COMMAND1 ARG1 ARG2 ARG3 ... ARGN
COMMAND2 ARG1 ARG2 ....

when piping;

COMMAND1 ARG1 ARG2 ARG3 ... ARGN | COMMAND2 ARG1 ARG2 ...

or

COMMAND1 ARG1 ARG2 ARG3 ... ARGN
| COMMAND2 ARG1 ARG2 ...

COMMAND1 ARG1 ARG2 ARG3 ... ARGN |


// any amount of newlines

COMMAND2 ARG1 ARG2 ...

When arguments need to include spaces, bars, newlines, or semicolons, they may either be quoted eg;

COMMAND "aaaaaaa   bbbbbbbbbb;;;;; ||| ;;;; wheeee" arg2;
or escaped with backslashes
COMMAND aaaaaa\ \ \ bbbbbbbbbbb\;\;\;\;\;\ \|\|\|\ \;\;\;\;\ wheeee arg2;

quotes may be "content", or 'content'
`content` isn't a quote, i don't know what to use that for yet. probably math (similar to bash's $()
'content' is a raw string, anything inside of it is just what is typed there. not greedy.
"content" is more fun, it allows for substitution, eg; "$variable content"
aswell as escapes, such as "this is followed by a newline\nthis is on a new line"
there's probably a standard for that i don't know about

variables are *NOT* expanded to multiple arguments by default, even if a direct substitution would
they are passed directly.
to expand them, either run the expand command (eg; expand var-name, or $var |= expand)
sidenote; '|=' is pipe into then equal, so value of variable as stdin, then set to stdout when complete.
uses buffer but write to any memory that's been sent to the command
|e= is just error
|&= is both
wow i tangent easily, that should go in the variable thingy
anyways that gives you a list, which you can then use the spread operator from nushell to use as multiple commands;
let list_of_args = [~/.bashrc, /home/larry/];
cp list_of_args...;

comments are 
COMMANDS AND ARGUMENTS BLAH BLAH BLAH # until newline
COMMANDS ARGUMENTS BLAH BLAH BLAH #* until closed
blah blah blah blah
commenty noises commenty noises
*# COMMANDS ARGUMENTS BLAH BLAH BLAH
later i'll add things like piping a file in using <, shell options to ignore or respect whitespace more or less, 
anyways time to go code this i guess
