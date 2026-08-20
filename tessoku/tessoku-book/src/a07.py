# [A07 - Event Attendance](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_g)
def solve(D: int, _N: int, LR: list[list[int]]):
    # 素朴にやると O(ND) > 10^10
    # O(D) は確定なので、N を減らすことを考える

    # 前日比を使った累積和で O(N + D) で解けるはず
    s = [0] * (D + 2)

    for l, r in LR:
        s[l] += 1
        s[r + 1] -= 1

    for i in range(D):
        s[i + 1] += s[i]

    for d in range(1, D + 1):
        print(s[d])


if __name__ == "__main__":
    D = int(input())
    N = int(input())
    LR = [list(map(int, input().split())) for _ in range(N)]

    solve(D, N, LR)
