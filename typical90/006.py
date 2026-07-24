from os import getenv


def solve(N: int, K: int, S: str):
    """
    >>> solve(7,3,"atcoder")
    acd
    >>> solve(14,5,"kittyonyourlap")
    inlap
    """

    ans = ""

    # 全探索 O(KNlogN)
    rest = S
    for k in range(K):
        # 辞書順でもっとも早い文字を選ぶ
        for c in sorted(rest):
            # 選んだ文字以降に K-k-1 文字以上残っているか
            if len(rest) - rest.index(c) >= K - k:
                ans += c
                rest = rest[rest.index(c) + 1 :]
                break

    print(ans)


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
