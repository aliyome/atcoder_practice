import string
from os import getenv


# [012 - Red Painting（★4）](https://atcoder.jp/contests/typical90/tasks/typical90_l)
def solve(H: int, W: int, Q: int, q: list[list[int]]):
    """
    >>> solve(3, 3, 10, [
    ...     [1, 2, 2],
    ...     [1, 1, 1],
    ...     [2, 1, 1, 2, 2],
    ...     [1, 3, 2],
    ...     [2, 1, 1, 2, 2],
    ...     [2, 2, 2, 3, 2],
    ...     [1, 2, 3],
    ...     [1, 2, 1],
    ...     [2, 1, 1, 2, 2],
    ...     [2, 1, 1, 3, 3]
    ... ])
    """
    map = [[0] * (W + 2) for _ in range(H + 2)]

    def dfs(r1, c1, r2, c2):
        stack = [(r1, c1)]
        visited = [[False] * (W + 2) for _ in range(H + 2)]

        while True:
            if not stack:
                break
            r, c = stack.pop()

            if map[r][c] == 0:
                return False

            if r == r2 and c == c2:
                return True

            if visited[r][c]:
                continue

            visited[r][c] = True

            if map[r + 1][c] == 1 and visited[r + 1][c] == False:
                stack.append((r + 1, c))
            if map[r - 1][c] == 1 and visited[r - 1][c] == False:
                stack.append((r - 1, c))
            if map[r][c + 1] == 1 and visited[r][c + 1] == False:
                stack.append((r, c + 1))
            if map[r][c - 1] == 1 and visited[r][c - 1] == False:
                stack.append((r, c - 1))

        return False

    for [t, *rest] in q:
        if t == 1:
            # 塗る
            r, c = rest
            map[r][c] = 1

        else:
            # クエリ
            r1, c1, r2, c2 = rest
            print("Yes" if dfs(r1, c1, r2, c2) else "No")


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
    H, W = map(int, input().split())
    Q = int(input())
    q = []
    for _ in range(Q):
        q.append(list(map(int, input().split())))

    solve(H, W, Q, q)
