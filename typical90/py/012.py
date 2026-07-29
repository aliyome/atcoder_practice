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
    No
    No
    Yes
    Yes
    No
    """
    map = [[0] * (W + 2) for _ in range(H + 2)]
    uf = UnionFind((H + 2) * (W + 2))

    for [t, *rest] in q:
        if t == 1:
            # 塗る
            r, c = rest
            map[r][c] = 1
            for dr, dc in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
                nr, nc = r + dr, c + dc
                if map[nr][nc] == 1:
                    uf.unite(r * (W + 2) + c, nr * (W + 2) + nc)

        else:
            # クエリ
            r1, c1, r2, c2 = rest
            ans = (
                uf.issame(r1 * (W + 2) + c1, r2 * (W + 2) + c2)
                and map[r1][c1] == 1
                and map[r2][c2] == 1
            )
            print("Yes" if ans else "No")


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


class UnionFind:
    def __init__(self, n: int):
        self.parent = [-1] * n  # 親ノードのindex。根の場合は-1
        self.rank = [0] * n  # 木の高さ
        self.size = [1] * n  # ノード数

    def root(self, a: int) -> int:
        if self.parent[a] == -1:
            return a
        self.parent[a] = self.root(self.parent[a])  # 経路圧縮
        return self.parent[a]

    def issame(self, a: int, b: int) -> bool:
        return self.root(a) == self.root(b)

    def unite(self, a: int, b: int) -> bool:
        ra, rb = self.root(a), self.root(b)
        if ra == rb:
            return False
        if self.rank[rb] < self.rank[ra]:
            ra, rb = rb, ra
        self.parent[ra] = rb
        self.rank[rb] = max(self.rank[rb], self.rank[ra] + 1)
        self.size[rb] += self.size[ra]
        return True


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
