# [A04 - Binary Representation 1](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_d)
def solve(N: int):
    print(bin(N)[2:].zfill(10))


if __name__ == "__main__":
    N = int(input())
    solve(N)
