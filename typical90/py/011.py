import string
from os import getenv


# [011 - Gravy Jobs（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_k)
def solve(N: int, DCS: list[list[int]]):
    """
    3
    7 3 200000
    3 2 100000
    5 3 150000
    >>> solve(3, [
    ...     [7, 3, 200000],
    ...     [3, 2, 100000],
    ...     [5, 3, 150000]
    ... ])
    350000
    >>> solve(8, [
    ...     [376, 640, 602876667],
    ...     [4015, 1868, 533609371],
    ...     [3330, 152, 408704870],
    ...     [1874, 798, 30417810],
    ...     [2, 1450, 40706045],
    ...     [3344, 1840, 801881841],
    ...     [2853, 1229, 5235900],
    ...     [458, 1277, 997429858]
    ... ])
    1744196082
    """
    # 全探索 O(N!): 5000! -> TLE
    # dp[i][day] := タスクiまでを検討してj日目までに得られる最大スコア
    MAX_DAYS = 5000
    dp = [[0] * (MAX_DAYS + 1) for _ in range(N + 1)]

    # タスクを締め切り昇順にソートする
    for i, [d, c, s] in enumerate(sorted(DCS, key=lambda x: x[0])):
        # day までの最大スコアを計算する
        for day in range(MAX_DAYS):
            # タスクiを選択しない場合は、最大スコアを次のタスク検討i+1に引き継ぐ
            dp[i + 1][day] = max(dp[i + 1][day], dp[i][day])
            # タスクが締め切り前ならスコアを加算する
            if day + c <= d:
                dp[i + 1][day + c] = max(dp[i + 1][day + c], dp[i][day] + s)

    print(max(max(dp)))


# pos[i][c] := Sのi文字目以降で文字cが最初に出現するindex
# 以下のように後ろ向きのDPで計算できる。
# pos[i][c] = pos[i+1][c] if S[i] != c else i
def calc_next_pos(S: str):
    N = len(S)
    res = [{c: 10**9 for c in string.ascii_lowercase} for _ in range(N + 1)]
    for i in range(N - 1, -1, -1):
        for c in string.ascii_lowercase:
            res[i][c] = res[i + 1][c]
        res[i][S[i]] = i
    return res


def binary_search_max(ok, ng, is_ok):
    while abs(ok - ng) > 1:
        mid = (ok + ng) // 2
        if is_ok(mid):
            ok = mid
        else:
            ng = mid
    return ok


if getenv("DOCTEST"):
    import doctest

    doctest.testmod()

if __name__ == "__main__" and not getenv("DOCTEST"):
    N = int(input())
    DCS = []
    for _ in range(N):
        d, c, s = map(int, input().split())
        DCS.append([d, c, s])
    solve(N, DCS)
