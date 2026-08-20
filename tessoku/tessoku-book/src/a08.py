# [A08 - Two Dimensional Sum](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_h)
def solve(H: int, W: int, X: list[list[int]], _Q: int, ABCD: list[list[int]]):
    # X: 0-indexed, S: 1-indexed, ABCD: 1-indexed
    # いもす法で累積和を計算する
    S = [[0] * (W + 2) for _ in range(H + 2)]
    # 縦
    for i in range(1, H + 1):
        for j in range(1, W + 1):
            S[i][j] += S[i - 1][j] + X[i][j]
    # 横
    for j in range(1, W + 1):
        for i in range(1, H + 1):
            S[i][j] += S[i][j - 1]
    # 左上+右下 - (左下+右上)
    for a, b, c, d in ABCD:
        ans = S[a - 1][b - 1] + S[c][d] - S[a - 1][d] - S[c][b - 1]
        print(ans)


if __name__ == "__main__":
    [H, W] = list(map(int, input().split()))
    X = [list(map(int, input().split())) for _ in range(H)]
    Q = int(input())
    ABCD = [list(map(int, input().split())) for _ in range(Q)]
    # X だけ 0-indexed だと不便なので 1-indexed に変換する
    X1 = [[0] * (W + 2) for _ in range(H + 2)]
    for i in range(H):
        for j in range(W):
            X1[i + 1][j + 1] = X[i][j]

    solve(H, W, X1, Q, ABCD)
