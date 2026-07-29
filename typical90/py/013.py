import string
from heapq import heappop, heappush
from os import getenv


# [013 - Passing（★5）](https://atcoder.jp/contests/typical90/tasks/typical90_m)
def solve(N: int, M: int, ABC: list[list[int]]):
    """
    >>> solve(7, 9, [
    ...     [1, 2, 2],
    ...     [1, 3, 3],
    ...     [2, 5, 2],
    ...     [3, 4, 1],
    ...     [3, 5, 4],
    ...     [4, 7, 5],
    ...     [5, 6, 1],
    ...     [5, 7, 6],
    ...     [6, 7, 3]
    ... ])
    8
    8
    9
    9
    8
    8
    8
    """

    edges = [[] for _ in range(N + 1)]
    for a, b, c in ABC:
        edges[a].append((b, c))
        edges[b].append((a, c))

    costs_from_start = dijkstra(edges, N, 1)
    costs_from_end = dijkstra(edges, N, N)

    for i in range(1, N + 1):
        print(costs_from_start[i] + costs_from_end[i])


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
    N, M = map(int, input().split())
    ABC = []
    for _ in range(M):
        ABC.append(list(map(int, input().split())))

    solve(N, M, ABC)
