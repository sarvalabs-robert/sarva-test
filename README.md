# Exercise: parse a line and calculate its `value`

A `line` is a string of arbitrary length that also includes words `incr` and `decr`.
The value is calculated as +1 for each occurrence of incr and -1 for decr.

## Example:
```
sdlkfjincrsadldecrklejinincrsdčlgk = 2
      ----    ----      ----

dslfkjdedecrxx = -1
        ----
````

## Expected result
A program that will use 3 sources of "lines", parse each line and calculate its value
and print the results:
- the number of all lines (should be 1000) and sum of all values
- also, print number of lines and sum only for lines with positive values and one for lines with negative values

To do the above you're expecred to write:
- a parser that reads a line and calculates the value
- for each source of data, get 1000 lines, evaluate each and print number of lines and sum;
  

## Data Sources
You have 3 sources of data that generate strings, line by line
1. `linegenerator`, a crate you can use to generate a random line
2. A file 1000lines.txt
3. An HTTP server `lineserver` you can run as a binary an retrieve the data from it

## End notes
Doing this in 45 minutes is hard (you're not even expected to finish every detail), but keep in mind that it's important to demonstrate your approach, including:
- ability to set up a development environment, run programs and use crates
- basic knowledge of string manipulation
- writing unit tests
- design for reusability of code
- have some general grasp how to do file and networking IO
