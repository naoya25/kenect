# 東京市区町村SVGの作り方

`apps/frontend/assets/tokyo.svg` は、東京モード用の白地図SVGです。
カード状のダミーではなく、東京都62自治体の実際の境界データからSVGの `<path>` を生成しています。

## 目的

ゲーム側では、現在地・使用済み・移動可能な市区町村を `data-code` で塗り分けます。
そのためSVGには、各自治体ごとに以下が必要です。

```xml
<path id="tokyo-shape-13101" class="tokyo-city" data-code="13101" ...>
```

`data-code` は `packages/shared/src/data/city/tokyo.rs` の JIS X 0402 コードと一致させます。

## データソース

使用したデータ:

- JapanCityGeoJson: https://github.com/niiyz/JapanCityGeoJson
- `geojson/13/*.json` が東京都の62自治体
- 元データは国土数値情報由来

Wikimedia Commons の `Blank_Tokyo.svg` も候補でしたが、線分中心のSVGで、ゲーム用に自治体単位で確実に塗り分けるには扱いづらかったため採用しませんでした。

## 生成方針

- `packages/shared/src/data/city/tokyo.rs` から、ゲームで使う62自治体のコード・日本語名・ローマ字を読む
- GitHub APIで `geojson/13` の一覧を取得する
- 各自治体のGeoJSONをダウンロードする
- GeoJSONの `Polygon` / `MultiPolygon` をSVGの `path d` に変換する
- 本土部、多摩地域、23区は同じパネルに配置する
- 島しょ部は実距離どおりに置くと見えなくなるため、伊豆諸島と小笠原を右側のインセットとして配置する
- `fill-rule="evenodd"` を使い、島や穴を含む複数リングを1自治体1パスで扱う

## 再生成手順

作業ディレクトリはリポジトリルートです。

```bash
curl -L --silent --fail \
  'https://api.github.com/repos/niiyz/JapanCityGeoJson/contents/geojson/13?ref=master' \
  -o /private/tmp/tokyo_listing.json

mkdir -p /private/tmp/tokyo_geojson

python3 - <<'PY'
import json
import pathlib
import subprocess

with open('/private/tmp/tokyo_listing.json') as f:
    files = json.load(f)

out = pathlib.Path('/private/tmp/tokyo_geojson')
for item in files:
    subprocess.run(
        ['curl', '-L', '--silent', '--fail', item['download_url'], '-o', str(out / item['name'])],
        check=True,
    )

print(len(list(out.glob('*.json'))))
PY
```

次に、GeoJSONからSVGを生成します。

```bash
python3 - <<'PY'
from pathlib import Path
import json, math, html, re

repo = Path.cwd()
geo_dir = Path('/private/tmp/tokyo_geojson')
out = repo / 'apps/frontend/assets/tokyo.svg'
shared = (repo / 'packages/shared/src/data/city/tokyo.rs').read_text()

entries = []
entry_re = re.compile(
    r'id:\s*([A-Za-z0-9_]+)\.id\(\),\n'
    r'\s*kind: RegionKind::City,\n'
    r'\s*parent: Some\(P::Tokyo.id\(\)\),\n'
    r'\s*name: "([^"]+)".*?\n'
    r'\s*kana: "([^"]+)".*?\n'
    r'\s*roman: "([^"]+)"',
    re.S,
)

for m in entry_re.finditer(shared):
    enum_name, name, _kana, roman = m.groups()
    code = re.search(rf'{re.escape(enum_name)} = (\d+)', shared).group(1)
    entries.append((code, name, roman))

if len(entries) != 62:
    raise SystemExit(f'expected 62 Tokyo entries, got {len(entries)}')

def geometry_rings(geom):
    if geom['type'] == 'Polygon':
        return geom['coordinates']
    if geom['type'] == 'MultiPolygon':
        rings = []
        for poly in geom['coordinates']:
            rings.extend(poly)
        return rings
    raise ValueError(geom['type'])

def distance(p, a, b):
    ax, ay = a; bx, by = b; px, py = p
    dx, dy = bx - ax, by - ay
    if dx == 0 and dy == 0:
        return math.hypot(px - ax, py - ay)
    t = max(0, min(1, ((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)))
    qx, qy = ax + t * dx, ay + t * dy
    return math.hypot(px - qx, py - qy)

def rdp(points, eps):
    if len(points) <= 3:
        return points
    start, end = points[0], points[-1]
    index, max_dist = 0, -1
    for i in range(1, len(points) - 1):
        dist = distance(points[i], start, end)
        if dist > max_dist:
            index, max_dist = i, dist
    if max_dist > eps:
        return rdp(points[:index + 1], eps)[:-1] + rdp(points[index:], eps)
    return [start, end]

def close_ring(ring):
    return ring if ring and ring[0] == ring[-1] else ring + [ring[0]]

features = {}
for code, name, roman in entries:
    data = json.loads((geo_dir / f'{code}.json').read_text())
    if len(data['features']) != 1:
        raise SystemExit(f'{code} has {len(data["features"])} features')
    features[code] = {
        'name': name,
        'roman': roman,
        'rings': geometry_rings(data['features'][0]['geometry']),
    }

mainland = {code for code, _, _ in entries if int(code) < 13361}
izu = {'13361', '13362', '13363', '13364', '13381', '13382', '13401', '13402'}
ogasawara = {'13421'}
panels = [
    ('mainland', mainland, 24, 24, 820, 610),
    ('islands', izu, 875, 54, 250, 360),
    ('ogasawara', ogasawara, 875, 468, 250, 170),
]

def bounds(codes):
    xs, ys = [], []
    for code in codes:
        for ring in features[code]['rings']:
            for lon, lat in ring:
                xs.append(lon)
                ys.append(lat)
    return min(xs), min(ys), max(xs), max(ys)

panel_proj = {}
for label, codes, x, y, w, h in panels:
    min_lon, min_lat, max_lon, max_lat = bounds(codes)
    scale = min(w / (max_lon - min_lon), h / (max_lat - min_lat))
    ox = x + (w - (max_lon - min_lon) * scale) / 2
    oy = y + (h - (max_lat - min_lat) * scale) / 2
    panel_proj[label] = (codes, min_lon, max_lat, scale, ox, oy)

def project(code, lon, lat):
    label = 'mainland' if code in mainland else 'islands' if code in izu else 'ogasawara'
    _, min_lon, max_lat, scale, ox, oy = panel_proj[label]
    return ox + (lon - min_lon) * scale, oy + (max_lat - lat) * scale

def path_for_feature(code):
    eps = 0.00035 if code in mainland else 0.0008
    parts = []
    for ring in features[code]['rings']:
        simple = close_ring(rdp(close_ring(ring), eps))
        coords = [project(code, lon, lat) for lon, lat in simple]
        first = coords[0]
        seg = [f'M{first[0]:.2f},{first[1]:.2f}']
        seg.extend(f'L{x:.2f},{y:.2f}' for x, y in coords[1:])
        seg.append('Z')
        parts.append(' '.join(seg))
    return ' '.join(parts)

lines = [
    '<svg class="geolonia-svg-map tokyo-svg-map" viewBox="0 0 1160 672" width="100%" height="100%" preserveAspectRatio="xMidYMid meet" xmlns="http://www.w3.org/2000/svg">',
    '  <title>Tokyo Municipalities Blank Map</title>',
    '  <desc>Tokyo municipality boundaries generated from JapanCityGeoJson 2020 data derived from MLIT National Land Numerical Information.</desc>',
    '  <rect class="tokyo-map-background" x="0" y="0" width="1160" height="672" fill="none"/>',
]

groups = [
    ('tokyo-map-mainland', mainland),
    ('tokyo-map-islands', izu),
    ('tokyo-map-ogasawara', ogasawara),
]

for group_class, codes in groups:
    lines.append(f'  <g class="tokyo-map-panel {group_class}">')
    for code, name, roman in entries:
        if code not in codes:
            continue
        lines.extend([
            f'    <path id="tokyo-shape-{code}" class="tokyo-city" data-code="{code}" fill="#eeeeee" fill-rule="evenodd" stroke="#111827" stroke-linejoin="round" stroke-width="1.1" d="{path_for_feature(code)}">',
            f'      <title>{html.escape(name)} / {html.escape(roman)}</title>',
            '    </path>',
        ])
    lines.append('  </g>')

lines.append('</svg>')
out.write_text('\n'.join(lines) + '\n')
PY
```

## 確認コマンド

```bash
python3 - <<'PY'
from pathlib import Path
import re

svg = Path('apps/frontend/assets/tokyo.svg').read_text()
codes = re.findall(r'data-code="(\d+)"', svg)
ids = re.findall(r'id="tokyo-shape-(\d+)"', svg)
print('paths', len(re.findall(r'<path\b', svg)))
print('data-code', len(codes), len(set(codes)))
print('ids', len(ids), len(set(ids)))
PY

rsvg-convert apps/frontend/assets/tokyo.svg -o /private/tmp/tokyo.svg.png
cargo check
```

期待値:

- `paths 62`
- `data-code 62 62`
- `ids 62 62`
- PNGへの変換が成功する
- `cargo check` が通る

## 注意点

- `data-code` は必ずゲーム側の `LocationId` と一致させる
- 島しょ部を実座標のまま1枚に収めると、本土部または島が実用にならないサイズになる
- `fill-rule="evenodd"` を外すと、複数リングや穴の描画が崩れることがある
- GeoJSONの点数が多いため、Ramer-Douglas-Peuckerで軽く簡略化する
- SVGのサイズが大きくなりすぎたら、`eps` を少し上げる。ただし境界が崩れやすいので、PNG化して見た目を確認する
