# [A06 - How Many Guests?](https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_ai)
def solve(N: int, Q: int, A: list[int], LR: list[list[int]]):
    # 累積和
    s = [0] * (N + 1)
    for i in range(N):
        s[i + 1] = s[i] + A[i]
    for i in range(Q):
        l, r = LR[i]
        print(s[r] - s[l - 1])


if __name__ == "__main__":
    [N, Q] = map(int, input().split())
    A = list(map(int, input().split()))
    LR = [list(map(int, input().split())) for _ in range(Q)]

    solve(N, Q, A, LR)
