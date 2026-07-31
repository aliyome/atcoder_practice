import string
from heapq import heappop, heappush
from os import getenv


# [015 - Don't be too close（★6）](https://atcoder.jp/contests/typical90/tasks/typical90_o)
def solve(N: int):
    """
    >>> solve(1)
    1
    >>> solve(2)
    3
    2
    >>> solve(3)
    7
    4
    3
    """

    MOD = 10**9 + 7

    combination = nCr(N, MOD)
    for k in range(1, N + 1):
        ans = 0
        for a in range(1, N + 1):
            p = N - (k - 1) * (a - 1)
            if p < a:
                break
            ans += combination(p, a)
            ans %= MOD

        print(ans)


def combination(n: int, r: int) -> int:
    if r < 0 or n < r:
        return 0
    if r == 0 or n == r:
        return 1
    r = min(r, n - r)
    numer = 1
    denom = 1
    for i in range(r):
        numer *= n - i
        denom *= i + 1
    return numer // denom


def nCr(max_n: int, mod: int = 10**9 + 7):
    fact = [1] * (max_n + 2)
    inv = [1] * (max_n + 2)

    for i in range(2, (max_n + 2)):
        fact[i] = fact[i - 1] * i
        fact[i] %= mod

    inv[-1] = pow(fact[-1], mod - 2, mod)
    for i in reversed(range(max_n + 2)):
        inv[i - 1] = inv[i] * i
        inv[i - 1] %= mod

    def impl(n: int, r: int) -> int:
        if r < 0 or n - r < 0:
            return 0
        return fact[n] * inv[n - r] % mod * inv[r] % mod

    return impl


# edges: [node][(next_node, cost)]
# cost は非負であることが前提. 負の場合ベルマンフォードを使う
# ヒープ（優先度付きキュー）:
#   heap[0] が常に最小値, O(1)で取得可能
#   要素の追加と最小値の削除は O(logN)
def dijkstra(edges: list[list[tuple[int, int]]], N: int, start: int) -> list[int]:
    costs = [10**9] * (N + 1)
    costs[start] = 0

    heap = [(0, start)]  # (累積cost, node)
    while heap:
        cost, node = heappop(heap)
        if costs[node] < cost:
            continue
        for next_node, cost in edges[node]:
            new_cost = costs[node] + cost
            if new_cost < costs[next_node]:
                costs[next_node] = new_cost
                heappush(heap, (new_cost, next_node))
    return costs


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
    N = int(input())

    solve(N)
