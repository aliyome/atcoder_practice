# [A05 - Three Cards](https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_e)
def solve(N: int, K: int):
    # 素朴に全探索 3000^3 = 27,000,000,000
    # 2つだけ決める 3000^2 = 9,000,000 < 10^7
    ans = 0
    for i in range(1, N + 1):
        for j in range(1, N + 1):
            if i + j >= K:
                continue
            k = K - i - j
            if k > N:
                continue
            ans += 1
    print(ans)


if __name__ == "__main__":
    [N, K] = map(int, input().split())
    solve(N, K)
