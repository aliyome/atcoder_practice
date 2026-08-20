# [A09 - Winter in ALGO Kingdom](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_i)
def solve(H: int, W: int, N: int, ABCD: list[list[int]]):
    yuki = [[0] * (W + 2) for _ in range(H + 2)]
    # 2次元累積和
    for a, b, c, d in ABCD:
        yuki[a][b] += 1
        yuki[c + 1][d + 1] += 1
        yuki[a][d + 1] -= 1
        yuki[c + 1][b] -= 1

    # 縦
    for h in range(1, H + 1):
        for w in range(1, W + 1):
            yuki[h][w] += yuki[h - 1][w]
    # 横
    for h in range(1, H + 1):
        for w in range(1, W + 1):
            yuki[h][w] += yuki[h][w - 1]

    for h in range(1, H + 1):
        for w in range(1, W + 1):
            print(yuki[h][w], end=" ")
        print()


if __name__ == "__main__":
    [H, W, N] = list(map(int, input().split()))
    ABCD = [list(map(int, input().split())) for _ in range(N)]

    solve(H, W, N, ABCD)
