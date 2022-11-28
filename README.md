### Problem

A function on a binary string T of length M is defined as follows:

F(T) = number of indices i (1≤i<M) such that Ti≠Ti+1.

You are given a binary string S of length N. You have to divide string S into two subsequences S1,S2 such that:

- Each character of string S belongs to one of S1 and S2.
- The characters of S1and S2.must occur in the order they appear in S.

Find the maximum possible value of F(S1)+F(S2).

**NOTE**: A binary string is a string that consists of characters `0` and `1`. One of the strings S1, S2 can be empty.

#### Input format

- The first line contains T denoting the number of test cases. The description of T test cases is as follows:
- For each test case:
- The first line contains N denoting the size of string S.
- The second line contains the string S.

#### Constraints

Sample Input
```
3
3
011
4
1100
5
11111
```
Sample Output
```
1
2
0
```

#### Explanation

##### The first test case
One possible division is S1=011,S2="" (empty string). Here F(S1)+F(S2)=1+0=1. There is no way to divide the given string to obtain a greater answer.
##### The second test case
The optimal division is S1=10,S2=10. Here F(S1)+F(S2)=1+1=2. There is no way to divide the given string to obtain a greater answer.
##### The third test case
For any possible division of S, F(S1)+F(S2)=0.