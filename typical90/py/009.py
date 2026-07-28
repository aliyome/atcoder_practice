import string
from math import atan2, degrees
from os import getenv


def solve(N: int, P: list[list[int]]):
    """
    >>> solve(3, [[0, 0], [0, 10], [10, 10]])
    90
    """

    # O(N^2 logN)
    max_angle = -1
    # 1点(Pi)は固定
    for i in range(N):
        # 偏角を計算してソートする
        angles = [(j, degrees(atan2(x, y))) for j, [x, y] in enumerate(P) if j != i]
        sorted_angles = sorted(angles, key=lambda x: x[1])

        # 2点目(Pj)を固定
        for j, angle in sorted_angles:
            # 偏角が最大になる点を二分探索で求める
            ok = 0
            ng = len(sorted_angles)
            calc_angle = -1
            while abs(ok - ng) > 1:
                mid = (ok + ng) // 2
                calc_angle = (angle - sorted_angles[mid][1] + 360) % 360
                if calc_angle < 180:
                    ok = mid
                    max_angle = max(max_angle, calc_angle)
                else:
                    ng = mid
            if calc_angle < max_angle:
                print(i, j, sorted_angles[ok][0])

    print(max_angle)


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
    P = []
    for _ in range(N):
        x, y = map(int, input().split())
        P.append((x, y))
    solve(N, P)
