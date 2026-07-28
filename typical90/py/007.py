import string
from os import getenv


def solve(N: int, A: list[int], Q: int, B: list[int]):
    """
    >>> solve(4, [4000, 4400, 5000, 3200], 3, [3312, 2992, 4229])
    112
    208
    171
    """
    # # 素朴に全探索 O(NQ): 3x10^5 * 3x10^5 = 9x10^10
    # for b in B:
    #     min_score = 10**9
    #     for a in A:
    #         min_score = min(min_score, abs(a - b))
    #     print(min_score)

    # 二分探索 O((N+Q)logN): NlogN + QlogN
    A_sorted = sorted(A)

    for b in B:
        ok = -1
        ng = N
        while abs(ok - ng) > 1:
            mid = (ok + ng) // 2
            if A_sorted[mid] < b:
                ok = mid
            else:
                ng = mid

        if ok == -1:
            print(abs(A_sorted[0] - b))
        elif ok == N - 1:
            print(abs(A_sorted[N - 1] - b))
        else:
            print(min(abs(A_sorted[ok] - b), abs(A_sorted[ok + 1] - b)))


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
    A = list(map(int, input().split()))
    Q = int(input())
    B = []
    for _ in range(Q):
        B.append(int(input()))
    solve(N, A, Q, B)
