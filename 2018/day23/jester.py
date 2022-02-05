from collections import OrderedDict, defaultdict, namedtuple
from functools import total_ordering
from heapq import heappop, heappush
from itertools import chain
import re

Bot = namedtuple('Bot', 'x y z r')

with open('src/input.txt') as puzzle_file:
    pattern = re.compile(r'pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)')
    bots = [Bot(*map(int, pattern.match(line).groups())) for line in puzzle_file]

max_r = max(bot.r for bot in bots)
print(
    max(
        sum(abs(b.x - a.x) + abs(b.y - a.y) + abs(b.z - a.z) <= a.r for b in bots) for a in bots
        if a.r == max_r))

@total_ordering
class Octa_ordering(object):
    def __lt__(self, other):
        return self.min < other.min or self.min == other.min and other.max < self.max

class Octa(Octa_ordering, namedtuple('Octa', ('min', 'max'))):
    def __new__(cls, *args, **kwargs):
        if 'bot' in kwargs:
            x, y, z, r = kwargs['bot']
            t, u, v, w = x + y + z, x + y - z, x - y - z, x - y + z
            return super(Octa, cls).__new__(cls, (t - r, u - r, v - r, w - r),
                                            (t + r, u + r, v + r, w + r))
        return super(Octa, cls).__new__(cls, *args, **kwargs)

    def intersect(self, other):
        c, d, e, f = self.min
        g, h, i, j = other.min
        k, l, m, n = self.max
        o, p, q, r = other.max
        s, t, u, v = max(c, g), max(d, h), max(e, i), max(f, j)
        w, x, y, z = min(k, o), min(l, p), min(m, q), min(n, r)
        return None if s > w or t > x or u > y or v > z else Octa((s, t, u, v), (w, x, y, z))

    def distance_to_origin(self):
        o, p, q, r = self.min
        s, t, u, v = self.max
        if o < s and p < t and q < u and r < v:
            w = min(abs(o), abs(s)) if o * s >= 0 else 0
            x = min(abs(p), abs(t)) if p * t >= 0 else 0
            y = min(abs(q), abs(u)) if q * u >= 0 else 0
            z = min(abs(r), abs(v)) if r * v >= 0 else 0
            return max(w, x, y, z)
        return min(
            abs((x + z) // 2) + abs((y - z) // 2) + abs((x - y) // 2)
            for x in range(o, s + 1) for y in range(p + ((p ^ x) & 1), t + 1, 2)
            for z in range(q + ((q ^ x) & 1), u + 1, 2) if r <= x - y + z <= v)

best_count = 0
octs = defaultdict(set)
for i, bot in enumerate(bots):
    octs[Octa(bot=bot)].add(i)
queue = [(0, (), OrderedDict((k, octs[k]) for k in sorted(octs)))]
while queue:
    n, _, rest = heappop(queue)
    if -n < best_count:
        break
    octa, n = rest.popitem()
    sub = defaultdict(set)
    for octa2, m in rest.items():
        octa3 = octa.intersect(octa2)
        if octa3 is not None:
            (n if octa == octa3 else sub[octa3]).update(m)
    if len(n) > best_count:
        best_count, best_distance = len(n), [octa]
    elif len(n) == best_count:
        best_distance.append(octa)
    m = frozenset(chain.from_iterable(rest.values()))
    heappush(queue, (-len(m), m, rest))
    rest = OrderedDict((k, sub[k].union(n)) for k in sorted(sub))
    m = frozenset(chain.from_iterable(rest.values()))
    heappush(queue, (-len(m), m, rest))
print(min(octa.distance_to_origin() for octa in best_distance))