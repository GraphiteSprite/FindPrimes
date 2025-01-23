https://www.codurance.com/katas/santas-allotment

Mattsi Jansky
See author's bio and posts
Twitter X logo
Santa wants to increase the number of Christmas trees that he has in his allotment.

Instructions
Input
Santa wants to plant n evergreen Christmas trees in his allotment.

His allotment is always a square of n plots by n plots.

One plot may contain a Christmas tree.

However, because of their magical root structure these evergreen trees will not grow if there is another one horizontally, vertically or diagonally aligned with the tree.

For example, given n = 4 and if we represent a tree with O and an empty lot with - (dash), then there is 1 valid output and 3 invalid.

Given a size n, try to find a valid solution for placing the trees in an allotment size of n.  If there is no valid solution, then return an empty array.

Logic
So, if n = 4, we can fit 4 Christmas trees on a 4 x 4 grid (see examples below).

However, n = 2, we cannot fit 2 trees onto a 2 x 2 grid following these rules, so we would return the empty array.

Valid Output - Example n = 4
Valid Output - santas allotment

Valid Output - Empty Array
empty array - allotment

Invalid Output - Example n = 4
1 Invalid Output - santas allotment

2 Invalid Output - santas allotment

3 Invalid Output - santas allotment

 