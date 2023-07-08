# Cubic Palindromes
This project was inspired by Domotro from Combo Class, who posted a video about there being one palindrome cube number where the root was not a palindrome, whilst the others were all palindromes (https://www.youtube.com/shorts/OubncQUm9dc). I wanted to see if there was another non-palindrome cube number, but I couldn't find one.

I noticed the single threaded performance was very poor, and a small percentage of my CPU was being used, so I made it multi-threaded. After a few cubed palindromes, I noticed that all of the remaining palindromes seemed to only contain 0s and 1s, so I began converting the iterator to a lower base to exclude numbers that probably won't be in the results. With the lower base, the numbers were growing much higher, so I had to start using Big Ints.

# Results
It appears that 2201 is the only non-palindrome palindrome cube if you're only counting ints.

There is also a pattern that these numbers follow once they start to only contain 1s and 0s:
For numbers with X digits, the first palindrome cube will be
* 100...001
If X is odd, then the next number will be:
* 100...010...001
If X is even, then the next number will be:
* 100...0110...001
Then the remaining numbers will have 2 inner `1`s that move closer and closer to the outside like so:
* 10001010001
* 10010001001
* 10100000101
* 11000000011
This pattern creates a growing and repeating upwards facing arrow, and it appears that there can only be a maximum of 4 `1`s in the number. This pattern would allow you to construct a palindrome cube without a calculator, such as this number
* 101000000000000000000000000000000000000000000000000000000000000000000000000000000000000000101

There are more non-palindrome palindrome cubes if you count decimals. Just pick almost any decimal palindrome, take the cubic root and it probably won't be a palindrome.

# Performance
I have tried different methods for converting to strings, changing bases, and checking palindromes, and have gotten it about as fast as it could get. The biggest improvement would be to use the GPU.
It would also be easier to count higher by only checking palindromes by constructing palindromes from the get-go.