import string
from bisect import bisect_left
from math import atan2, degrees
from os import getenv


def solve(N: int, CP: list[list[int]], Q: int, LR: list[list[int]]):
    """
    7
    1 72
    2 78
    2 94
    1 23
    2 89
    1 40
    1 75
    1
    2 6

    >>> solve(7, [
    ...     [1, 72],
    ...     [2, 78],
    ...     [2, 94],
    ...     [1, 23],
    ...     [2, 89],
    ...     [1, 40],
    ...     [1, 75]
    ... ], 1, [
    ...     [2, 6]
    ... ])
    63 261
    """
    sum = [[0, 0] for _ in range(N + 1)]
    for i in range(N):
        c, p = CP[i]
        sum[i + 1][0] = sum[i][0] + (p if c == 1 else 0)
        sum[i + 1][1] = sum[i][1] + (p if c == 2 else 0)
    for l, r in LR:
        print(sum[r][0] - sum[l - 1][0], sum[r][1] - sum[l - 1][1])


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
    CP = []
    for _ in range(N):
        c, p = map(int, input().split())
        CP.append((c, p))
    Q = int(input())
    LR = []
    for _ in range(Q):
        l, r = map(int, input().split())
        LR.append((l, r))
    solve(N, CP, Q, LR)
