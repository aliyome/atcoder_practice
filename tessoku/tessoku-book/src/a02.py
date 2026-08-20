# [A02 - Linear Search](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_b)
def solve(_N: int, X: int, A: list[int]):
    if X in A:
        print("Yes")
    else:
        print("No")


if __name__ == "__main__":
    [N, X] = map(int, input().split())
    A = list(map(int, input().split()))
    solve(N, X, A)
