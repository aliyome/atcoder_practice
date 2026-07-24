from os import getenv


def solve(N, L, K, A):
    """
    >>> solve(3, 34, 1, [8, 13, 26])
    13
    >>> solve(7, 45, 2, [7, 11, 16, 20, 28, 34, 38])
    12
    """

    def is_ok(x):
        count = 0
        last = 0
        for a in A:
            if a - last >= x:
                count += 1
                last = a
        if L - last >= x:
            count += 1
        return count >= K + 1

    return binary_search_max(0, L + 1, is_ok)


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
    N, L = map(int, input().split())
    K = int(input())
    A = list(map(int, input().split()))

    print(solve(N, L, K, A))
