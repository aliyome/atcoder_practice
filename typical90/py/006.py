import string
from os import getenv


def solve(N: int, K: int, S: str):
    """
    >>> solve(7,3,"atcoder")
    acd
    >>> solve(14,5,"kittyonyourlap")
    inlap
    """
    next_pos = calc_next_pos(S)

    # 貪欲法
    res = ""
    current_pos = 0  # Sのcurrent_pos文字目以降から選ぶ
    for i in range(K):
        for c in string.ascii_lowercase:
            p = next_pos[current_pos][c]
            if p + (K - i) <= N:
                res += c
                current_pos = p + 1
                break

    print(res)


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
    N, K = map(int, input().split())
    S = input()
    solve(N, K, S)
