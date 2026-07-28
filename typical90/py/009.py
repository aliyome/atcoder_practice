import string
from bisect import bisect_left
from math import atan2, degrees
from os import getenv

# bisect_left(arr, target): arrの中でtarget以上の最小のindexを返す


def solve(N: int, P: list[list[int]]):
    """
    >>> solve(3, [[0, 0], [0, 10], [10, 10]])
    90.0
    """

    max_angle = -1
    # 中心点(Pa)は固定
    for a in range(N):
        ax, ay = P[a]
        # Paから見た各点への偏角を計算してソートする
        angles = [
            (degrees(atan2(ay - by, ax - bx)) + 360) % 360
            for b, [bx, by] in enumerate(P)
            if b != a
        ]
        sorted_angles = sorted(angles)
        M = len(sorted_angles)

        # 視点(Pb)を固定
        for angle in sorted_angles:
            # 偏角が最大になる点を二分探索で求める
            # 最も大きい角度は angle + 180
            target = (angle + 180) % 360
            idx = bisect_left(sorted_angles, target)
            # idxがMを超える場合は0に戻す
            for i in [idx % M, (idx - 1) % M]:
                diff = abs(sorted_angles[i] - angle)
                diff = 360 - diff if diff > 180 else diff
                max_angle = max(max_angle, diff)

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
