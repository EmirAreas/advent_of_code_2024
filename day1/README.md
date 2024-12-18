
# Advent of Code 2024 - Day 1: Historian Hysteria

## Problem Description

The Chief Historian of the North Pole has gone missing, and the Elvish Senior Historians are racing to find him before Christmas! Their search begins with two incomplete lists of historically significant location IDs discovered in the Chief's office. To aid in their investigation, you must reconcile these two lists and solve the following puzzles:

### Part One: Total Distance Between Lists
The two lists of location IDs appear very different at first glance, but perhaps the differences are small. To determine this:

1. Sort both the left and right lists of location IDs.
2. Pair up numbers in the lists by matching the smallest number in one list with the smallest number in the other, the second smallest with the second smallest, and so on.
3. Compute the distance for each pair as the absolute difference between the two numbers.
4. Sum up all the distances.

For example:
```
Left List:  [3, 4, 2, 1, 3, 3]
Right List: [4, 3, 5, 3, 9, 3]

Sorted Left List:  [1, 2, 3, 3, 3, 4]
Sorted Right List: [3, 3, 3, 4, 5, 9]

Pairs and Distances:
- (1, 3): Distance = 2
- (2, 3): Distance = 1
- (3, 3): Distance = 0
- (3, 4): Distance = 1
- (3, 5): Distance = 2
- (4, 9): Distance = 5

Total Distance = 2 + 1 + 0 + 1 + 2 + 5 = 11
```

Your task is to calculate the **total distance** for the given input lists.

### Part Two: Total Similarity Score
The differences between the lists may not be as significant as they seem, as many location IDs appear in both lists. To measure this similarity:

1. For each number in the left list, count how many times it appears in the right list.
2. Multiply the number by its frequency in the right list.
3. Add up these values to calculate the total similarity score.

For example:
```
Left List:  [3, 4, 2, 1, 3, 3]
Right List: [4, 3, 5, 3, 9, 3]

Calculations:
- Number 3 appears 3 times in the right list: 3 * 3 = 9
- Number 4 appears 1 time in the right list: 4 * 1 = 4
- Number 2 appears 0 times in the right list: 2 * 0 = 0
- Number 1 appears 0 times in the right list: 1 * 0 = 0
- Number 3 appears 3 times in the right list: 3 * 3 = 9
- Number 3 appears 3 times in the right list: 3 * 3 = 9

Total Similarity Score = 9 + 4 + 0 + 0 + 9 + 9 = 31
```

Your task is to calculate the **total similarity score** for the given input lists.

---

## Input Format
Your input consists of two lists of integers:
- A **left list** of location IDs.
- A **right list** of location IDs.

Both lists are of equal length.

---

## How to Run the Solution
1. Clone this repository:
   ```bash
   git clone https://github.com/EmirAread/advent_of_code_2024.git
   cd advent_of_code_2024/day1
   ```

2. Run the solution script:
   ```bash
   cargo run
   ```

3. Replace `input.txt` with your puzzle input file. The file should contain two columns of integers separated by whitespace.

---

## Example Input
```
3 4
4 3
2 5
1 3
3 9
3 3
```

## Example Output
**Part One (Total Distance):**  
`11`

**Part Two (Similarity Score):**  
`31`

---

## Repository Structure
- `historian_hysteria.py`: Python solution for Day 1 puzzles.
- `input.txt`: Example input file.
- `README.md`: This documentation.

---

## Solution Notes
- The solution efficiently handles sorting and frequency counting to compute the results for both parts.
- Time complexity:
  - **Part One:** \(O(n \log n)\) due to sorting.
  - **Part Two:** \(O(n)\) due to frequency counting.

Happy coding and Merry Christmas! 🎄
