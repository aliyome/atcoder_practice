import string
from os import getenv


def solve(N: int, S: str):
    """
    >>> solve(10, "attcordeer")
    4
    """
    # 素朴に全探索 O(2^N): 2^10^5 = 10^30 -> TLE
    # 耳 DP
    # dp[i][j] := Sのi文字目まで見て、Tのj文字目までを作れる組み合わせの数
    # dp[i][j] =
    #   dp[i-1][j]                if S[i] != T[j]
    #   dp[i-1][j] + dp[i-1][j-1] if S[i] == T[j]
    # 計算しやすくするために S の先頭にダミー文字を追加する
    s = " " + S
    t = " atcoder"
    dp = [[0] * (len(t) + 1) for _ in range(len(s) + 1)]
    # S を 0 文字使って T を 0 文字作る方法は 1 通り（何も選ばない）
    dp[0][0] = 1
    for i in range(1, len(s)):
        for j in range(len(t)):
            dp[i][j] = dp[i - 1][j]
            if s[i] == t[j]:
                dp[i][j] += dp[i - 1][j - 1]
                dp[i][j] %= 10**9 + 7
    print(dp[N][len(t) - 1])


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
    S = input()
    solve(N, S)
